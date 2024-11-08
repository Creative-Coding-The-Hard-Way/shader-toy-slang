use {
    anyhow::{Context, Result},
    ash::vk,
    clap::Parser,
    glfw::{Action, Key, Window, WindowEvent},
    std::sync::Arc,
    sts::{
        app::{app_main, App},
        graphics::{
            ortho_projection,
            vulkan::{
                raii, CPUBuffer, FrameStatus, FramesInFlight,
                PresentImageStatus, Swapchain, VulkanContext,
            },
            Sprite, SpriteLayer,
        },
        trace,
    },
};

#[derive(Parser, Debug)]
struct Args {}

/// A sprites demo.
struct Sprites {
    world_layer: SpriteLayer,
    sprites: CPUBuffer<Sprite>,

    // Vulkan resources
    frames_in_flight: FramesInFlight,
    renderpass: raii::RenderPass,
    framebuffers: Vec<raii::Framebuffer>,
    swapchain: Arc<Swapchain>,
    swapchain_needs_rebuild: bool,
    ctx: Arc<VulkanContext>,
}

impl App for Sprites {
    type Args = Args;

    fn new(window: &mut glfw::Window, _args: Self::Args) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);

        let ctx = VulkanContext::new(window)
            .with_context(trace!("Unable to create device!"))?;

        let (w, h) = window.get_framebuffer_size();
        let swapchain = Swapchain::new(ctx.clone(), (w as u32, h as u32), None)
            .with_context(trace!("Unable to create swapchain!"))?;

        let frames_in_flight = FramesInFlight::new(ctx.clone(), 2)
            .with_context(trace!("Unable to create frames_in_flight!"))?;

        let renderpass = create_renderpass(ctx.device.clone(), &swapchain)?;
        let framebuffers = create_framebuffers(&ctx, &renderpass, &swapchain)?;

        let world_layer = SpriteLayer::builder()
            .ctx(ctx.clone())
            .frames_in_flight(&frames_in_flight)
            .render_pass(&renderpass)
            .swapchain(&swapchain)
            .projection(ortho_projection(w as f32 / h as f32, 10.0))
            .build()?;

        let mut sprites =
            CPUBuffer::allocate(&ctx, 1, vk::BufferUsageFlags::STORAGE_BUFFER)?;

        unsafe {
            // SAFE: because the data is not being used yet
            sprites.write_data(
                0,
                &[Sprite {
                    pos: [0.0, 0.0],
                    size: [1.0, 1.0],
                    uv_pos: [0.0, 0.0],
                    uv_size: [1.0, 1.0],
                    tint: [1.0, 1.0, 1.0, 1.0],
                    angle: 0.0,
                    texture: 0,
                }],
            )?;
        };

        Ok(Self {
            world_layer,
            sprites,
            frames_in_flight,
            renderpass,
            framebuffers,
            swapchain,
            swapchain_needs_rebuild: false,
            ctx,
        })
    }

    fn handle_event(
        &mut self,
        window: &mut Window,
        event: WindowEvent,
    ) -> Result<()> {
        if let glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) =
            event
        {
            window.set_should_close(true);
        }

        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.swapchain_needs_rebuild {
            self.rebuild_swapchain(window)?;
            self.world_layer.rebuild_swapchain_resources(
                &self.swapchain,
                &self.renderpass,
                &self.frames_in_flight,
            )?;
            let (w, h) = window.get_framebuffer_size();
            self.world_layer
                .set_projection(&ortho_projection(w as f32 / h as f32, 10.0));
        }

        let frame = match self.frames_in_flight.start_frame(&self.swapchain)? {
            FrameStatus::FrameStarted(frame) => frame,
            FrameStatus::SwapchainNeedsRebuild => {
                self.swapchain_needs_rebuild = true;
                return Ok(());
            }
        };

        let clear_colors = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 0.0],
            },
        }];
        unsafe {
            self.ctx.cmd_begin_render_pass(
                frame.command_buffer(),
                &vk::RenderPassBeginInfo {
                    render_pass: self.renderpass.raw,
                    framebuffer: self.framebuffers
                        [frame.swapchain_image_index() as usize]
                        .raw,
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: self.swapchain.extent(),
                    },
                    clear_value_count: clear_colors.len() as u32,
                    p_clear_values: clear_colors.as_ptr(),
                    ..Default::default()
                },
                vk::SubpassContents::INLINE,
            );
        }

        SpriteLayer::begin(&mut self.world_layer, &frame)?
            .draw((self.sprites.buffer(), 1))?
            .finish();

        unsafe {
            self.ctx.cmd_end_render_pass(frame.command_buffer());
        }

        if self
            .frames_in_flight
            .present_frame(&self.swapchain, frame)?
            == PresentImageStatus::SwapchainNeedsRebuild
        {
            self.swapchain_needs_rebuild = true;
        }

        Ok(())
    }
}

impl Sprites {
    fn rebuild_swapchain(&mut self, window: &mut Window) -> Result<()> {
        self.swapchain_needs_rebuild = false;

        unsafe {
            // wait for all pending work to finish
            self.ctx.device_wait_idle()?;
        }

        self.swapchain = {
            let (w, h) = window.get_framebuffer_size();
            Swapchain::new(
                self.ctx.clone(),
                (w as u32, h as u32),
                Some(self.swapchain.raw()),
            )?
        };

        self.renderpass =
            create_renderpass(self.ctx.device.clone(), &self.swapchain)?;
        self.framebuffers =
            create_framebuffers(&self.ctx, &self.renderpass, &self.swapchain)?;

        Ok(())
    }
}

fn create_renderpass(
    device: Arc<raii::Device>,
    swapchain: &Swapchain,
) -> Result<raii::RenderPass> {
    let attachments = [vk::AttachmentDescription {
        format: swapchain.format(),
        samples: vk::SampleCountFlags::TYPE_1,
        load_op: vk::AttachmentLoadOp::CLEAR,
        store_op: vk::AttachmentStoreOp::STORE,
        stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
        initial_layout: vk::ImageLayout::UNDEFINED,
        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        ..Default::default()
    }];
    let color_attachment = [vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    }];
    let subpasses = [vk::SubpassDescription {
        pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
        input_attachment_count: 0,
        p_input_attachments: std::ptr::null(),
        color_attachment_count: color_attachment.len() as u32,
        p_color_attachments: color_attachment.as_ptr(),
        p_resolve_attachments: std::ptr::null(),
        p_depth_stencil_attachment: std::ptr::null(),
        preserve_attachment_count: 0,
        p_preserve_attachments: std::ptr::null(),
        ..Default::default()
    }];
    let dependencies = [
        vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        },
        vk::SubpassDependency {
            src_subpass: 0,
            dst_subpass: vk::SUBPASS_EXTERNAL,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags::empty(),
            dependency_flags: vk::DependencyFlags::empty(),
        },
    ];
    raii::RenderPass::new(
        device,
        &vk::RenderPassCreateInfo {
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: subpasses.len() as u32,
            p_subpasses: subpasses.as_ptr(),
            dependency_count: dependencies.len() as u32,
            p_dependencies: dependencies.as_ptr(),
            ..Default::default()
        },
    )
}

/// Creates one framebuffer per swapchain image view.
///
/// Framebuffers must be replaced when the swapchain is rebuilt.
fn create_framebuffers(
    cxt: &VulkanContext,
    render_pass: &raii::RenderPass,
    swapchain: &Swapchain,
) -> Result<Vec<raii::Framebuffer>> {
    let mut framebuffers = vec![];
    let vk::Extent2D { width, height } = swapchain.extent();
    for image_view in swapchain.image_views() {
        let create_info = vk::FramebufferCreateInfo {
            render_pass: render_pass.raw,
            attachment_count: 1,
            p_attachments: &image_view.raw,
            width,
            height,
            layers: 1,
            ..Default::default()
        };
        framebuffers
            .push(raii::Framebuffer::new(cxt.device.clone(), &create_info)?);
    }
    Ok(framebuffers)
}

impl Drop for Sprites {
    fn drop(&mut self) {
        self.frames_in_flight
            .wait_for_all_frames_to_complete()
            .expect("Error while shutting down!");
    }
}

fn main() {
    app_main::<Sprites>();
}
