use {
    crate::{
        graphics::vulkan::{
            raii, AcquireImageStatus, Device, PresentImageStatus, Swapchain,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FrameStatus {
    /// Indicates that the frame is started.
    ///
    /// The command buffer is still owned by the FramesInFlight and does not
    /// need to be freed by the caller.
    FrameStarted(Frame),

    /// Indicates that the swapchain needs to be rebuilt.
    SwapchainNeedsRebuild,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Frame {
    command_buffer: vk::CommandBuffer,
    swapchain_image_index: u32,
    frame_index: usize,
}

impl Frame {
    pub fn command_buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    pub fn swapchain_image_index(&self) -> u32 {
        self.swapchain_image_index
    }

    pub fn frame_index(&self) -> usize {
        self.frame_index
    }
}

/// Per-frame synchronization primitives.
struct FrameSync {
    swapchain_image_acquired: raii::Semaphore,
    color_attachment_written: raii::Semaphore,
    graphics_commands_complete: raii::Fence,
    command_pool: raii::CommandPool,
    command_buffer: vk::CommandBuffer,
}

/// All of the synchronization primitives required to manage multiple frames
/// in flight.
pub struct FramesInFlight {
    frames: Vec<FrameSync>,
    frame_index: usize,
    device: Arc<Device>,
}

impl Drop for FramesInFlight {
    fn drop(&mut self) {
        let fences = self
            .frames
            .iter()
            .map(|frame_sync| frame_sync.graphics_commands_complete.raw)
            .collect::<Vec<vk::Fence>>();
        unsafe {
            self.device
                .wait_for_fences(&fences, true, u64::MAX)
                .unwrap();
            self.device.reset_fences(&fences).unwrap();
            self.device.device_wait_idle().unwrap();
        }
    }
}

impl FramesInFlight {
    pub fn new(device: Arc<Device>, frame_count: usize) -> Result<Self> {
        let mut frames = Vec::with_capacity(frame_count);
        for index in 0..frame_count {
            let command_pool = raii::CommandPool::new(
                device.logical_device.clone(),
                &vk::CommandPoolCreateInfo {
                    flags: vk::CommandPoolCreateFlags::TRANSIENT,
                    queue_family_index: device.graphics_queue_family_index,
                    ..Default::default()
                },
            )
            .with_context(trace!(
                "Error while creating command pool for frame {}",
                index
            ))?;
            let command_buffer = unsafe {
                device.allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo {
                        command_pool: command_pool.raw,
                        level: vk::CommandBufferLevel::PRIMARY,
                        command_buffer_count: 1,
                        ..Default::default()
                    },
                )?[0]
            };
            frames.push(FrameSync {
                swapchain_image_acquired: raii::Semaphore::new(
                    device.logical_device.clone(),
                    &vk::SemaphoreCreateInfo::default(),
                )
                .with_context(trace!(
                    "Error while creating semaphore for frame {}",
                    index
                ))?,
                color_attachment_written: raii::Semaphore::new(
                    device.logical_device.clone(),
                    &vk::SemaphoreCreateInfo::default(),
                )
                .with_context(trace!(
                    "Error while creating semaphore for frame {}",
                    index
                ))?,
                graphics_commands_complete: raii::Fence::new(
                    device.logical_device.clone(),
                    &vk::FenceCreateInfo {
                        flags: vk::FenceCreateFlags::SIGNALED,
                        ..Default::default()
                    },
                )
                .with_context(trace!(
                    "Error creating fence for frame {}",
                    index
                ))?,
                command_pool,
                command_buffer,
            });
        }
        Ok(Self {
            frames,
            frame_index: 0,
            device,
        })
    }

    /// Get the total number of configured frames in flight.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Starts the next frame in flight.
    ///
    /// This method *can* block if all frames are in flight. It will block until
    /// the next frame is available.
    ///
    /// # Returns
    ///
    /// - FrameStatus::FrameStarted(vk::CommandBuffer): A non-owning copy of the
    ///   graphics command buffer for the current frame. The caller does not
    ///   need to call begin_command_buffer or end_command_buffer, both are
    ///   handled automatically.
    /// - FrameStatus::SwapchainNeedsRebuild: Indicates that the swapchain needs
    ///   to be rebuilt before a frame can be acquired.
    pub fn start_frame(
        &mut self,
        swapchain: &Swapchain,
    ) -> Result<FrameStatus> {
        self.frame_index = (self.frame_index + 1) % self.frames.len();

        // Start the Frame
        let frame_sync = &self.frames[self.frame_index];
        unsafe {
            // Wait for the last frame's submission to complete, if its still
            // running.
            self.device
                .wait_for_fences(
                    &[frame_sync.graphics_commands_complete.raw],
                    true,
                    u64::MAX,
                )
                .with_context(trace!(
                    "Error while waiting for frame's commands to complete!"
                ))?;
            self.device
                .reset_fences(&[frame_sync.graphics_commands_complete.raw])
                .with_context(trace!(
                    "Error while resetting the frame's fence!"
                ))?;
        };

        // Acquire the next Swapchain image
        let status = swapchain
            .acquire_image(frame_sync.swapchain_image_acquired.raw)
            .with_context(trace!(
                "Error while acquiring swapchain image for frame!"
            ))?;
        let swapchain_image_index = match status {
            AcquireImageStatus::ImageAcquired(index) => index,
            _ => {
                return Ok(FrameStatus::SwapchainNeedsRebuild);
            }
        };

        // Start the Frame's command buffer.
        unsafe {
            self.device
                .reset_command_pool(
                    frame_sync.command_pool.raw,
                    vk::CommandPoolResetFlags::empty(),
                )
                .with_context(trace!(
                    "Error while resetting command buffer for frame!"
                ))?;
            self.device
                .begin_command_buffer(
                    frame_sync.command_buffer,
                    &vk::CommandBufferBeginInfo {
                        flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                        ..Default::default()
                    },
                )
                .with_context(trace!(
                    "Error while beginning the frame's command buffer!"
                ))?;
        };

        Ok(FrameStatus::FrameStarted(Frame {
            command_buffer: frame_sync.command_buffer,
            swapchain_image_index,
            frame_index: self.frame_index,
        }))
    }

    pub fn present_frame(
        &mut self,
        swapchain: &Swapchain,
        frame: Frame,
    ) -> Result<PresentImageStatus> {
        let frame_sync = &self.frames[frame.frame_index()];
        unsafe {
            self.device
                .end_command_buffer(frame_sync.command_buffer)
                .with_context(trace!(
                    "Error while ending the command buffer!"
                ))?;

            let wait_stage = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
            self.device
                .queue_submit(
                    self.device.graphics_queue,
                    &[vk::SubmitInfo {
                        wait_semaphore_count: 1,
                        p_wait_semaphores: &frame_sync
                            .swapchain_image_acquired
                            .raw,
                        p_wait_dst_stage_mask: &wait_stage,
                        command_buffer_count: 1,
                        p_command_buffers: &frame_sync.command_buffer,
                        signal_semaphore_count: 1,
                        p_signal_semaphores: &frame_sync
                            .color_attachment_written
                            .raw,
                        ..Default::default()
                    }],
                    frame_sync.graphics_commands_complete.raw,
                )
                .with_context(trace!(
                    "Error while submitting frame commands!"
                ))?;
        }

        swapchain
            .present_image(
                frame_sync.color_attachment_written.raw,
                frame.swapchain_image_index(),
            )
            .with_context(trace!("Error while presenting swapchain image!"))
    }
}
