use {
    crate::{
        graphics::vulkan::{raii, Device},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// A utility for synchronously submitting commands to the GPU.
#[derive(Debug)]
pub struct SyncCommands {
    command_pool: raii::CommandPool,
    command_buffer: vk::CommandBuffer,
    fence: raii::Fence,
    device: Arc<Device>,
}

impl SyncCommands {
    pub fn new(device: Arc<Device>) -> Result<Self> {
        let command_pool = raii::CommandPool::new(
            device.logical_device.clone(),
            &vk::CommandPoolCreateInfo {
                flags: vk::CommandPoolCreateFlags::TRANSIENT,
                queue_family_index: device.graphics_queue_family_index,
                ..Default::default()
            },
        )
        .with_context(trace!("Unable to create command pool!"))?;
        let command_buffer = unsafe {
            device
                .allocate_command_buffers(&vk::CommandBufferAllocateInfo {
                    command_pool: command_pool.raw,
                    level: vk::CommandBufferLevel::PRIMARY,
                    command_buffer_count: 1,
                    ..Default::default()
                })
                .with_context(trace!(
                    "Unable to allocate the command buffer!"
                ))?[0]
        };
        let fence = raii::Fence::new(
            device.logical_device.clone(),
            &vk::FenceCreateInfo::default(),
        )?;
        Ok(Self {
            command_pool,
            command_buffer,
            fence,
            device,
        })
    }

    pub fn submit_and_wait(
        &self,
        build_commands: impl FnOnce(vk::CommandBuffer) -> Result<()>,
    ) -> Result<()> {
        unsafe {
            self.device
                .reset_command_pool(
                    self.command_pool.raw,
                    vk::CommandPoolResetFlags::empty(),
                )
                .with_context(trace!("Error while resetting command pool!"))?;

            self.device.begin_command_buffer(
                self.command_buffer,
                &vk::CommandBufferBeginInfo {
                    flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                    ..Default::default()
                },
            )?;
        }

        build_commands(self.command_buffer).with_context(trace!(
            "Error while adding commands to the buffer!"
        ))?;

        unsafe {
            self.device
                .end_command_buffer(self.command_buffer)
                .with_context(trace!("Error while ending command buffer!"))?;

            self.device
                .queue_submit(
                    self.device.graphics_queue,
                    &[vk::SubmitInfo {
                        wait_semaphore_count: 0,
                        p_wait_semaphores: std::ptr::null(),
                        p_wait_dst_stage_mask: std::ptr::null(),
                        command_buffer_count: 1,
                        p_command_buffers: &self.command_buffer,
                        signal_semaphore_count: 0,
                        p_signal_semaphores: std::ptr::null(),
                        ..Default::default()
                    }],
                    self.fence.raw,
                )
                .with_context(trace!("Error while submitting commands!"))?;

            self.device
                .wait_for_fences(&[self.fence.raw], true, u64::MAX)
                .with_context(trace!(
                    "Error while waiting for commands to finish!"
                ))?;
            self.device
                .reset_fences(&[self.fence.raw])
                .with_context(trace!("Error while resetting fences!"))?;
        }

        Ok(())
    }
}
