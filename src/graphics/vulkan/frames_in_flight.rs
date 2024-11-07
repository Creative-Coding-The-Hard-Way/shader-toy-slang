use {
    crate::{
        graphics::vulkan::{
            raii, AcquireImageStatus, PresentImageStatus, Swapchain,
            VulkanContext,
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

/// A Frame is guaranteed to be synchronized such that no two frames with the
/// same frame_index can be in-flight on the GPU at the same time.
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

/// The primary synchronization mechanism for managing multiple in-flight
/// frames.
///
/// There can be 1-N frames in flight for the application, decided at the time
/// of construction. This is independent from the number of swapchain images,
/// though there is little-to-no benefit to having more frames in flight than
/// swapchain images.
///
/// Synchronization is performed such that when [Self::start_frame] returns, all
/// commands submitted to that frame are guaranteed to be complete. Thus, the
/// application can keep N copies of a resource and use the frame_index to
/// prevent synchronization errors.
pub struct FramesInFlight {
    frames: Vec<FrameSync>,
    frame_index: usize,
    cxt: Arc<VulkanContext>,
}

impl FramesInFlight {
    /// Creates a new instance with `frame_count` frames.
    pub fn new(cxt: Arc<VulkanContext>, frame_count: usize) -> Result<Self> {
        let mut frames = Vec::with_capacity(frame_count);
        for index in 0..frame_count {
            let command_pool = raii::CommandPool::new(
                cxt.device.clone(),
                &vk::CommandPoolCreateInfo {
                    flags: vk::CommandPoolCreateFlags::TRANSIENT,
                    queue_family_index: cxt.graphics_queue_family_index,
                    ..Default::default()
                },
            )
            .with_context(trace!(
                "Error while creating command pool for frame {}",
                index
            ))?;
            let command_buffer = unsafe {
                cxt.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
                    command_pool: command_pool.raw,
                    level: vk::CommandBufferLevel::PRIMARY,
                    command_buffer_count: 1,
                    ..Default::default()
                })?[0]
            };
            frames.push(FrameSync {
                swapchain_image_acquired: raii::Semaphore::new(
                    cxt.device.clone(),
                    &vk::SemaphoreCreateInfo::default(),
                )
                .with_context(trace!(
                    "Error while creating semaphore for frame {}",
                    index
                ))?,
                color_attachment_written: raii::Semaphore::new(
                    cxt.device.clone(),
                    &vk::SemaphoreCreateInfo::default(),
                )
                .with_context(trace!(
                    "Error while creating semaphore for frame {}",
                    index
                ))?,
                graphics_commands_complete: raii::Fence::new(
                    cxt.device.clone(),
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
            cxt,
        })
    }

    /// Get the total number of configured frames in flight.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Blocks until all submitted commands for all frames have completed.
    pub fn wait_for_all_frames_to_complete(&self) -> Result<()> {
        let fences = self
            .frames
            .iter()
            .map(|frame_sync| frame_sync.graphics_commands_complete.raw)
            .collect::<Vec<vk::Fence>>();
        unsafe {
            self.cxt
                .wait_for_fences(&fences, true, u64::MAX)
                .with_context(trace!(
                    "Error while waiting for all frames to finish rendering!"
                ))?;
        }
        Ok(())
    }

    /// Starts the next frame in flight.
    ///
    /// This method *can* block if all frames are in flight. It will block until
    /// the next frame is available.
    ///
    /// # Returns
    ///
    /// A [FrameStatus] containing one of:
    /// * A [Frame] that must be returned to [Self::present_frame]
    /// * A flag indicating that the Swapchain needs to be rebuilt before the
    ///   next frame.
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
            self.cxt
                .wait_for_fences(
                    &[frame_sync.graphics_commands_complete.raw],
                    true,
                    u64::MAX,
                )
                .with_context(trace!(
                    "Error while waiting for frame's commands to complete!"
                ))?;
            self.cxt
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
            self.cxt
                .reset_command_pool(
                    frame_sync.command_pool.raw,
                    vk::CommandPoolResetFlags::empty(),
                )
                .with_context(trace!(
                    "Error while resetting command buffer for frame!"
                ))?;
            self.cxt
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

    /// Queues the [Frame]'s command buffer and swapchain presentation.
    pub fn present_frame(
        &mut self,
        swapchain: &Swapchain,
        frame: Frame,
    ) -> Result<PresentImageStatus> {
        let frame_sync = &self.frames[frame.frame_index()];
        unsafe {
            self.cxt
                .end_command_buffer(frame_sync.command_buffer)
                .with_context(trace!(
                    "Error while ending the command buffer!"
                ))?;

            let wait_stage = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
            self.cxt
                .queue_submit(
                    self.cxt.graphics_queue,
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

impl Drop for FramesInFlight {
    fn drop(&mut self) {
        self.wait_for_all_frames_to_complete().unwrap();
        unsafe {
            self.cxt.device_wait_idle().unwrap();
        }
    }
}
