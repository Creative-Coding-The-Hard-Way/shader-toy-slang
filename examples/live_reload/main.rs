//! Run with the command:
//!   cargo run --example live_reload -- <args>

use {
    anyhow::{Context, Result},
    ash::vk,
    clap::Parser,
    glfw::{Action, Key, WindowEvent},
    std::{
        path::PathBuf,
        sync::Arc,
        time::{Duration, Instant},
    },
    sts::{
        app::{app_main, App, FullscreenToggle},
        graphics::{
            vulkan::{
                raii, Device, FrameStatus, FramesInFlight, OwnedBlock,
                PresentImageStatus, Swapchain,
            },
            FullscreenQuad, Recompiler,
        },
        trace,
    },
};

#[derive(Parser, Debug, Eq, PartialEq)]
#[command(version, about, long_about=None)]
struct Args {
    /// The fragment shader source path.
    #[arg(short, long)]
    pub frag_shader: PathBuf,

    /// Additional files/directories to watch. Any change to the file (or
    /// files, if a directory) will trigger a shader rebuild.
    #[arg(short, long)]
    pub additional_watch_dir: Vec<PathBuf>,
}

// This can be accepted in the fragment shader with code like:
//
//   struct FrameData {
//       float2 mouse_pos;
//       float2 screen_size;
//       float dt;
//       float time;
//   };
//
//   [[vk_binding(0, 0)]]
//   ConstantBuffer<FrameData> frame;
//
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(C)]
pub struct FrameData {
    pub mouse_pos: [f32; 2],
    pub screen_size: [f32; 2],
    pub dt: f32,
    pub time: f32,
}

struct LiveReload {
    start_time: Instant,
    last_frame: Instant,
    fragment_shader_compiler: Recompiler,
    fullscreen_toggle: FullscreenToggle,

    device: Arc<Device>,

    frames_in_flight: FramesInFlight,
    swapchain: Arc<Swapchain>,
    swapchain_needs_rebuild: bool,

    render_pass: raii::RenderPass,
    framebuffers: Vec<raii::Framebuffer>,

    fullscreen_quad: FullscreenQuad<FrameData>,
}

impl App for LiveReload {
    type Args = Args;

    fn new(window: &mut glfw::Window, args: Args) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);
        window.set_title(
            args.frag_shader
                .parent()
                .and_then(|a| a.to_str())
                .unwrap_or("shader-toy-slang"),
        );

        let device = Device::new(window)?;
        log::trace!("Created device: {:#?}", device);

        let fragment_shader_compiler =
            Recompiler::new(&args.frag_shader, &args.additional_watch_dir)
                .with_context(trace!(
                    "Unable to start the fragment shader compiler!"
                ))?;

        let (w, h) = window.get_framebuffer_size();
        let swapchain =
            Swapchain::new(device.clone(), (w as u32, h as u32), None)?;
        log::trace!("Created swapchain: {:#?}", swapchain);

        let frames_in_flight = FramesInFlight::new(device.clone(), 3)?;

        let render_pass = create_renderpass(&device, &swapchain)?;
        let framebuffers =
            create_framebuffers(&device, &render_pass, &swapchain)?;

        let fullscreen_quad = FullscreenQuad::new(
            device.clone(),
            fragment_shader_compiler.current_shader_bytes(),
            &frames_in_flight,
            &swapchain,
            &render_pass,
        )?;

        Ok(Self {
            start_time: Instant::now(),
            last_frame: Instant::now(),
            fragment_shader_compiler,
            fullscreen_toggle: FullscreenToggle::new(window),

            device,

            frames_in_flight,
            swapchain,
            swapchain_needs_rebuild: false,

            render_pass,
            framebuffers,
            fullscreen_quad,
        })
    }

    fn handle_event(
        &mut self,
        window: &mut glfw::Window,
        event: glfw::WindowEvent,
    ) -> Result<()> {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                window.set_should_close(true);
            }
            WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                self.fullscreen_toggle.toggle_fullscreen(window)?;
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, window: &mut glfw::Window) -> Result<()> {
        std::thread::sleep(Duration::from_millis(4));
        if self.swapchain_needs_rebuild {
            self.swapchain_needs_rebuild = false;
            self.rebuild_swapchain(window)?;
        }

        if self.fragment_shader_compiler.check_for_update()? {
            self.frames_in_flight
                .wait_for_all_frames_to_complete()
                .with_context(trace!("Error while waiting for frames"))?;
            self.fullscreen_quad.rebuild_pipeline(
                &self.swapchain,
                &self.render_pass,
                self.fragment_shader_compiler.current_shader_bytes(),
            )?;
        }

        let frame = match self.frames_in_flight.start_frame(&self.swapchain)? {
            FrameStatus::FrameStarted(command_buffer) => command_buffer,
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

        unsafe {
            let clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            };
            self.device.cmd_begin_render_pass(
                frame.command_buffer(),
                &vk::RenderPassBeginInfo {
                    render_pass: self.render_pass.raw,
                    framebuffer: self.framebuffers
                        [frame.swapchain_image_index() as usize]
                        .raw,
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

            self.fullscreen_quad.draw(&frame, frame_data)?;

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

impl LiveReload {
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

        log::trace!("{:#?}", self.swapchain);
        self.fragment_shader_compiler.check_for_update()?;
        self.fullscreen_quad.rebuild_pipeline(
            &self.swapchain,
            &self.render_pass,
            self.fragment_shader_compiler.current_shader_bytes(),
        )?;
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
    app_main::<LiveReload>();
}
