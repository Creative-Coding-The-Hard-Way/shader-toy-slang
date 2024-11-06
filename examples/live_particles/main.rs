use {
    anyhow::{Context, Result},
    ash::vk,
    clap::Parser,
    glfw::{Action, Key, Window, WindowEvent},
    std::{path::PathBuf, sync::Arc, time::Instant},
    sts::{
        app::{app_main, App},
        graphics::{
            vulkan::{
                raii, Device, FrameStatus, FramesInFlight, PresentImageStatus,
                Swapchain,
            },
            Particles, Recompiler,
        },
        trace,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(C)]
pub struct FrameData {
    pub mouse_pos: [f32; 2],
    pub screen_size: [f32; 2],
    pub dt: f32,
    pub time: f32,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    kernel_source: PathBuf,

    #[arg(short, long)]
    init_source: PathBuf,
}

struct LiveParticles {
    start_time: Instant,
    last_frame: Instant,

    frames_in_flight: FramesInFlight,
    renderpass: raii::RenderPass,
    swapchain: Arc<Swapchain>,
    framebuffers: Vec<raii::Framebuffer>,
    swapchain_needs_rebuild: bool,

    particles: Particles<FrameData>,

    kernel_compiler: Recompiler,
    init_compiler: Recompiler,

    device: Arc<Device>,
}

impl App for LiveParticles {
    type Args = Args;

    fn new(window: &mut glfw::Window, _args: Self::Args) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);

        let device = Device::new(window)
            .with_context(trace!("Unable to create device!"))?;

        let swapchain = {
            let (w, h) = window.get_framebuffer_size();
            Swapchain::new(device.clone(), (w as u32, h as u32), None)
                .with_context(trace!("Unable to create swapchain!"))?
        };

        let frames_in_flight = FramesInFlight::new(device.clone(), 2)
            .with_context(trace!("Unable to create frames_in_flight!"))?;

        let kernel_compiler = Recompiler::new(&_args.kernel_source, &[])?;
        let init_compiler = Recompiler::new(&_args.init_source, &[])?;

        let renderpass =
            create_renderpass(device.logical_device.clone(), &swapchain)?;
        let framebuffers =
            create_framebuffers(&device, &renderpass, &swapchain)?;

        let particles = Particles::builder()
            .device(device.clone())
            .frames_in_flight(&frames_in_flight)
            .swapchain(&swapchain)
            .render_pass(&renderpass)
            .kernel_bytes(kernel_compiler.current_shader_bytes())
            .init_bytes(init_compiler.current_shader_bytes())
            .build()
            .with_context(trace!("Unable to create particles!"))?;

        log::info!("{:#?}", particles);

        Ok(Self {
            start_time: Instant::now(),
            last_frame: Instant::now(),
            frames_in_flight,
            framebuffers,
            swapchain,
            swapchain_needs_rebuild: false,
            particles,
            kernel_compiler,
            init_compiler,
            device,
            renderpass,
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
        }

        if self.kernel_compiler.check_for_update()?
            || self.init_compiler.check_for_update()?
        {
            self.particles.compute_updated(
                self.kernel_compiler.current_shader_bytes(),
                self.init_compiler.current_shader_bytes(),
                &self.frames_in_flight,
            )?;
            self.start_time = Instant::now();
        }

        let frame = match self.frames_in_flight.start_frame(&self.swapchain)? {
            FrameStatus::FrameStarted(frame) => frame,
            FrameStatus::SwapchainNeedsRebuild => {
                self.swapchain_needs_rebuild = true;
                return Ok(());
            }
        };

        let frame_data = {
            let now = Instant::now();
            let dt = (now - self.last_frame).as_secs_f32();
            self.last_frame = now;

            let (x_64, y_64) = window.get_cursor_pos();
            let (w_i32, h_i32) = window.get_size();

            let (x, y) = (x_64 as f32, y_64 as f32);
            let (w, h) = (w_i32 as f32, h_i32 as f32);

            FrameData {
                mouse_pos: [
                    sts::map(x, 0.0..w, -1.0..1.0),
                    sts::map(y, 0.0..h, 1.0..-1.0),
                ],
                screen_size: [w, h],
                time: (now - self.start_time).as_secs_f32(),
                dt,
            }
        };
        self.particles.tick(&frame, frame_data)?;

        let clear_colors = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 0.0],
            },
        }];
        unsafe {
            self.device.cmd_begin_render_pass(
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

        self.particles.draw(&frame)?;

        unsafe {
            self.device.cmd_end_render_pass(frame.command_buffer());
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

impl LiveParticles {
    fn rebuild_swapchain(&mut self, window: &mut Window) -> Result<()> {
        self.swapchain_needs_rebuild = false;

        unsafe {
            // wait for all pending work to finish
            self.device.device_wait_idle()?;
        }

        self.swapchain = {
            let (w, h) = window.get_framebuffer_size();
            Swapchain::new(
                self.device.clone(),
                (w as u32, h as u32),
                Some(self.swapchain.raw()),
            )?
        };

        self.renderpass = create_renderpass(
            self.device.logical_device.clone(),
            &self.swapchain,
        )?;
        self.framebuffers = create_framebuffers(
            &self.device,
            &self.renderpass,
            &self.swapchain,
        )?;

        self.particles
            .swapchain_rebuilt(&self.swapchain, &self.renderpass)?;

        Ok(())
    }
}

fn create_renderpass(
    logical_device: Arc<raii::Device>,
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
        logical_device,
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
    device: &Device,
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
        framebuffers.push(raii::Framebuffer::new(
            device.logical_device.clone(),
            &create_info,
        )?);
    }
    Ok(framebuffers)
}

fn main() {
    app_main::<LiveParticles>();
}
