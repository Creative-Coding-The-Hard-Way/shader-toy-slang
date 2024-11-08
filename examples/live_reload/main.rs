//! Run with the command:
//!   cargo run --example live_reload -- <args>

use {
    anyhow::{Context, Result},
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
                FrameStatus, FramesInFlight, PresentImageStatus, Swapchain,
                VulkanContext,
            },
            FullscreenQuad, Recompiler, SwapchainColorPass, TextureLoader,
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

    /// An additional texture to provide to the shader.
    #[arg(short, long)]
    pub texture: Vec<PathBuf>,
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

    cxt: Arc<VulkanContext>,

    frames_in_flight: FramesInFlight,
    swapchain: Arc<Swapchain>,
    swapchain_needs_rebuild: bool,
    color_pass: SwapchainColorPass,

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

        let cxt = VulkanContext::new(window)?;
        log::trace!("Created device: {:#?}", cxt);

        let fragment_shader_compiler = Recompiler::new(
            cxt.clone(),
            &args.frag_shader,
            &args.additional_watch_dir,
        )
        .with_context(trace!(
            "Unable to start the fragment shader compiler!"
        ))?;

        let (w, h) = window.get_framebuffer_size();
        let swapchain =
            Swapchain::new(cxt.clone(), (w as u32, h as u32), None)?;
        log::trace!("Created swapchain: {:#?}", swapchain);

        let frames_in_flight = FramesInFlight::new(cxt.clone(), 3)?;

        let color_pass = SwapchainColorPass::new(cxt.clone(), &swapchain)?;

        let textures = {
            let mut loader = TextureLoader::new(cxt.clone())?;
            let mut textures = vec![];
            for path in &args.texture {
                let texture = loader.load_texture(path).with_context(
                    trace!("Error while loading texture {:?}", path),
                )?;
                textures.push(texture);
            }
            textures
        };

        let fullscreen_quad = FullscreenQuad::builder()
            .cxt(cxt.clone())
            .fragment_shader(fragment_shader_compiler.shader())
            .frames_in_flight(&frames_in_flight)
            .swapchain(&swapchain)
            .render_pass(color_pass.renderpass())
            .textures(textures)
            .build()?;

        Ok(Self {
            start_time: Instant::now(),
            last_frame: Instant::now(),
            fragment_shader_compiler,
            fullscreen_toggle: FullscreenToggle::new(window),

            cxt,

            frames_in_flight,
            swapchain,
            swapchain_needs_rebuild: false,
            color_pass,

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
                self.color_pass.renderpass(),
                self.fragment_shader_compiler.shader(),
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

        self.color_pass
            .begin_render_pass(&frame, [0.0, 0.0, 0.0, 0.0]);

        self.fullscreen_quad.draw(&frame, frame_data)?;

        self.color_pass.end_render_pass(&frame);

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
            self.cxt.device_wait_idle()?;
        }

        let (w, h) = window.get_framebuffer_size();
        self.swapchain = Swapchain::new(
            self.cxt.clone(),
            (w as u32, h as u32),
            Some(self.swapchain.raw()),
        )?;

        self.color_pass =
            SwapchainColorPass::new(self.cxt.clone(), &self.swapchain)?;

        log::trace!("{:#?}", self.swapchain);
        self.fragment_shader_compiler.check_for_update()?;
        self.fullscreen_quad.rebuild_pipeline(
            &self.swapchain,
            self.color_pass.renderpass(),
            self.fragment_shader_compiler.shader(),
        )?;
        Ok(())
    }
}

pub fn main() {
    app_main::<LiveReload>();
}
