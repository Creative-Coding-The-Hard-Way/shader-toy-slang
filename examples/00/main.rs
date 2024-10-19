mod buffer;
mod pipeline;

use {
    anyhow::Result,
    ash::vk,
    buffer::CPUBuffer,
    glfw::{Action, Key, WindowEvent},
    pipeline::{
        create_descriptor_pool, create_descriptor_set_layout, FrameData,
    },
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
    descriptor_set_layout: raii::DescriptorSetLayout,
    _descriptor_pool: raii::DescriptorPool,
    descriptor_set: vk::DescriptorSet,
    uniform_buffer: CPUBuffer,
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
            &self.descriptor_set_layout,
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
        let descriptor_set_layout = create_descriptor_set_layout(&device)?;
        let framebuffers =
            create_framebuffers(&device, &render_pass, &swapchain)?;
        let (pipeline, pipeline_layout) = pipeline::create_pipeline(
            &device,
            &swapchain,
            &render_pass,
            &descriptor_set_layout,
        )?;
        let descriptor_pool = create_descriptor_pool(&device)?;
        let descriptor_set = unsafe {
            let allocate_info = vk::DescriptorSetAllocateInfo {
                descriptor_pool: descriptor_pool.raw,
                descriptor_set_count: 1,
                p_set_layouts: &descriptor_set_layout.raw,
                ..Default::default()
            };
            device.allocate_descriptor_sets(&allocate_info)?[0]
        };
        let mut uniform_buffer = CPUBuffer::allocate(
            &device,
            std::mem::size_of::<FrameData>() as u64,
        )?;
        uniform_buffer.write(FrameData {
            mouse_pos: [0.0, 0.0],
        })?;

        unsafe {
            let buffer_info = vk::DescriptorBufferInfo {
                buffer: uniform_buffer.buffer.raw,
                offset: 0,
                range: std::mem::size_of::<FrameData>() as u64,
            };
            device.update_descriptor_sets(
                &[vk::WriteDescriptorSet {
                    dst_set: descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: &buffer_info,
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                }],
                &[],
            );
        };

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
            descriptor_set_layout,
            _descriptor_pool: descriptor_pool,
            descriptor_set,
            uniform_buffer,
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
        let (x, y) = window.get_cursor_pos();
        let (w, h) = window.get_size();

        self.uniform_buffer.write(FrameData {
            mouse_pos: [
                (x.clamp(0.0, w as f64) / w as f64) as f32,
                1.0 - (y.clamp(0.0, h as f64) / h as f64) as f32,
            ],
        })?;

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
                    float32: [0.0, 0.0, 0.0, 0.0],
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

            self.device.cmd_bind_descriptor_sets(
                self.command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                0,
                &[self.descriptor_set],
                &[],
            );

            self.device.cmd_draw(self.command_buffer, 3, 1, 0, 0);

            self.device.cmd_end_render_pass(self.command_buffer);

            self.device.end_command_buffer(self.command_buffer)?;

            let wait_stage = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
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
