mod pipeline;

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
    render_pass: raii::RenderPass,
    framebuffers: Vec<raii::Framebuffer>,
    swapchain_needs_rebuild: bool,
    pipeline: raii::Pipeline,
    pipeline_layout: raii::PipelineLayout,
}

impl Example {
    fn rebuild_swapchain(&mut self, window: &mut glfw::Window) -> Result<()> {
        unsafe {
            // wait for all pending work to finish
            self.device.device_wait_idle()?;
        }

        self.framebuffers.clear();

        let (w, h) = window.get_framebuffer_size();
        self.swapchain = Swapchain::new(
            self.device.clone(),
            (w as u32, h as u32),
            Some(self.swapchain.raw.raw),
        )?;

        self.render_pass = create_renderpass(&self.device, &self.swapchain)?;
        self.framebuffers = create_framebuffers(
            &self.device,
            &self.render_pass,
            &self.swapchain,
        )?;

        log::info!("{:#?}", self.swapchain);

        let (pipeline, pipeline_layout) = pipeline::create_pipeline(
            &self.device,
            &self.swapchain,
            &self.render_pass,
        )?;
        self.pipeline = pipeline;
        self.pipeline_layout = pipeline_layout;

        Ok(())
    }
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
        let swapchain =
            Swapchain::new(device.clone(), (w as u32, h as u32), None)?;

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

        let render_pass = create_renderpass(&device, &swapchain)?;
        let framebuffers =
            create_framebuffers(&device, &render_pass, &swapchain)?;
        let (pipeline, pipeline_layout) =
            pipeline::create_pipeline(&device, &swapchain, &render_pass)?;

        Ok(Self {
            device,
            swapchain,
            command_pool,
            command_buffer,
            render_pass,
            framebuffers,
            swapchain_needs_rebuild: false,
            pipeline,
            pipeline_layout,
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

    fn update(&mut self, window: &mut glfw::Window) -> Result<()> {
        if self.swapchain_needs_rebuild {
            self.swapchain_needs_rebuild = false;
            self.rebuild_swapchain(window)?;
        }

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
                self.swapchain_needs_rebuild = true;
                log::warn!("Swapchain needs rebuilt!");
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

            let clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.2, 0.2, 0.5, 1.0],
                },
            };
            self.device.cmd_begin_render_pass(
                self.command_buffer,
                &vk::RenderPassBeginInfo {
                    render_pass: self.render_pass.raw,
                    framebuffer: self.framebuffers[index as usize].raw,
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: self.swapchain.extent,
                    },
                    clear_value_count: 1,
                    p_clear_values: &clear_value,
                    ..Default::default()
                },
                vk::SubpassContents::INLINE,
            );

            self.device.cmd_bind_pipeline(
                self.command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.raw,
            );

            self.device.cmd_draw(self.command_buffer, 3, 1, 0, 0);

            self.device.cmd_end_render_pass(self.command_buffer);

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
            self.swapchain_needs_rebuild = true;
        }

        unsafe {
            self.device.device_wait_idle()?;
        }

        Ok(())
    }
}

/// Create a renderpass for the application.
///
/// The renderpass has a single subpass with a single color attachment for the
/// swapchain image.
fn create_renderpass(
    device: &Device,
    swapchain: &Swapchain,
) -> Result<raii::RenderPass> {
    let attachment_description = vk::AttachmentDescription {
        format: swapchain.format.format,
        samples: vk::SampleCountFlags::TYPE_1,
        load_op: vk::AttachmentLoadOp::CLEAR,
        store_op: vk::AttachmentStoreOp::STORE,
        stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
        initial_layout: vk::ImageLayout::UNDEFINED,
        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        ..Default::default()
    };
    let attachment_reference = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };
    let subpass_description = vk::SubpassDescription {
        pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
        input_attachment_count: 0,
        p_input_attachments: std::ptr::null(),
        color_attachment_count: 1,
        p_color_attachments: &attachment_reference,
        ..Default::default()
    };
    let subpass_dependency = vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        dst_subpass: 0,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask: vk::AccessFlags::empty(),
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        dependency_flags: vk::DependencyFlags::empty(),
    };
    let create_info = vk::RenderPassCreateInfo {
        attachment_count: 1,
        p_attachments: &attachment_description,
        subpass_count: 1,
        p_subpasses: &subpass_description,
        dependency_count: 1,
        p_dependencies: &subpass_dependency,
        ..Default::default()
    };
    raii::RenderPass::new(device.logical_device.clone(), &create_info)
}

/// Creates one framebuffer per swapchain image view.
///
/// Framebuffers must be replaced when the swapchain is rebuilt.
fn create_framebuffers(
    device: &Device,
    render_pass: &raii::RenderPass,
    swapchain: &Swapchain,
) -> Result<Vec<raii::Framebuffer>> {
    let mut framebuffers = vec![];
    let vk::Extent2D { width, height } = swapchain.extent;
    for image_view in &swapchain.image_views {
        let create_info = vk::FramebufferCreateInfo {
            render_pass: render_pass.raw,
            attachment_count: 1,
            p_attachments: &image_view.raw,
            width,
            height,
            layers: 1,
            ..Default::default()
        };
        framebuffers.push(raii::Framebuffer::new(
            device.logical_device.clone(),
            &create_info,
        )?);
    }
    Ok(framebuffers)
}

pub fn main() {
    app_main::<Example>();
}
