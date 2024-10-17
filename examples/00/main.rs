use {
    anyhow::Result,
    ash::vk,
    glfw::{Action, Key, WindowEvent},
    std::sync::Arc,
    sts::{
        app::{app_main, App},
        graphics::vulkan::{
            raii, AcquireImageStatus, Device, PresentImageStatus, Swapchain,
        },
    },
};

struct Example {
    swapchain: Arc<Swapchain>,
    device: Arc<Device>,
    command_pool: raii::CommandPool,
    command_buffer: vk::CommandBuffer,
}

impl App for Example {
    fn new(window: &mut glfw::Window) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);

        let device = Device::new(window)?;

        log::debug!("Created device: {:#?}", device);

        let (w, h) = window.get_framebuffer_size();
        let swapchain = Swapchain::new(device.clone(), (w as u32, h as u32))?;

        log::debug!("Created swapchain: {:#?}", swapchain);

        let command_pool = raii::CommandPool::new(
            device.logical_device.clone(),
            &vk::CommandPoolCreateInfo {
                flags: vk::CommandPoolCreateFlags::empty(),
                queue_family_index: device.graphics_queue_family_index,
                ..Default::default()
            },
        )?;

        let command_buffer = unsafe {
            device.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
                command_pool: command_pool.raw,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: 1,
                ..Default::default()
            })?[0]
        };

        Ok(Self {
            device,
            swapchain,
            command_pool,
            command_buffer,
        })
    }

    fn handle_event(
        &mut self,
        window: &mut glfw::Window,
        event: glfw::WindowEvent,
    ) -> Result<()> {
        if let WindowEvent::Key(Key::Escape, _, Action::Release, _) = event {
            window.set_should_close(true);
        }
        Ok(())
    }

    fn update(&mut self, _window: &mut glfw::Window) -> Result<()> {
        unsafe {
            self.device.reset_command_pool(
                self.command_pool.raw,
                vk::CommandPoolResetFlags::empty(),
            )?;
        }

        let image_acquired = raii::Semaphore::new(
            self.device.logical_device.clone(),
            &vk::SemaphoreCreateInfo::default(),
        )?;
        let graphics_complete = raii::Semaphore::new(
            self.device.logical_device.clone(),
            &vk::SemaphoreCreateInfo::default(),
        )?;

        let status = self.swapchain.acquire_image(image_acquired.raw)?;
        let index = match status {
            AcquireImageStatus::ImageAcquired(index) => index,
            _ => {
                log::warn!("AOEU");
                return Ok(());
            }
        };

        unsafe {
            self.device.begin_command_buffer(
                self.command_buffer,
                &vk::CommandBufferBeginInfo {
                    flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                    ..Default::default()
                },
            )?;

            self.device.cmd_pipeline_barrier(
                self.command_buffer,
                vk::PipelineStageFlags::ALL_GRAPHICS,
                vk::PipelineStageFlags::ALL_GRAPHICS,
                vk::DependencyFlags::empty(),
                &[], // memory barriers
                &[], // buffer barriers
                &[vk::ImageMemoryBarrier {
                    src_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ,
                    dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                    old_layout: vk::ImageLayout::UNDEFINED,
                    new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                    src_queue_family_index: self
                        .device
                        .graphics_queue_family_index,
                    dst_queue_family_index: self
                        .device
                        .graphics_queue_family_index,
                    image: self.swapchain.images[index as usize],
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    ..Default::default()
                }],
            );

            self.device.end_command_buffer(self.command_buffer)?;

            let wait_stage = vk::PipelineStageFlags::ALL_GRAPHICS;
            self.device.queue_submit(
                self.device.graphics_queue,
                &[vk::SubmitInfo {
                    wait_semaphore_count: 1,
                    p_wait_semaphores: &image_acquired.raw,
                    p_wait_dst_stage_mask: &wait_stage,
                    command_buffer_count: 1,
                    p_command_buffers: &self.command_buffer,
                    signal_semaphore_count: 1,
                    p_signal_semaphores: &graphics_complete.raw,
                    ..Default::default()
                }],
                vk::Fence::null(),
            )?;
        }

        let result =
            self.swapchain.present_image(graphics_complete.raw, index)?;
        if result == PresentImageStatus::SwapchainNeedsRebuild {
            log::warn!("needs rebuild after present");
        }

        unsafe {
            self.device.device_wait_idle()?;
        }

        Ok(())
    }
}

pub fn main() {
    app_main::<Example>();
}
