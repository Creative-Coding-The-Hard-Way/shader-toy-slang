use {
    anyhow::{Context, Result},
    clap::Parser,
    glfw::{Action, Key, Window, WindowEvent},
    std::{path::PathBuf, sync::Arc, time::Instant},
    sts::{
        app::{app_main, App},
        graphics::{
            vulkan::{
                FrameStatus, FramesInFlight, PresentImageStatus, Swapchain,
                VulkanContext,
            },
            Particles, Recompiler, SwapchainColorPass,
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
    swapchain: Arc<Swapchain>,
    swapchain_needs_rebuild: bool,
    color_pass: SwapchainColorPass,

    particles: Particles<FrameData>,

    kernel_compiler: Recompiler,
    init_compiler: Recompiler,

    cxt: Arc<VulkanContext>,
}

impl App for LiveParticles {
    type Args = Args;

    fn new(window: &mut glfw::Window, _args: Self::Args) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);

        let cxt = VulkanContext::new(window)
            .with_context(trace!("Unable to create device!"))?;

        let swapchain = {
            let (w, h) = window.get_framebuffer_size();
            Swapchain::new(cxt.clone(), (w as u32, h as u32), None)
                .with_context(trace!("Unable to create swapchain!"))?
        };

        let frames_in_flight = FramesInFlight::new(cxt.clone(), 2)
            .with_context(trace!("Unable to create frames_in_flight!"))?;

        let kernel_compiler =
            Recompiler::new(cxt.clone(), &_args.kernel_source, &[])?;
        let init_compiler =
            Recompiler::new(cxt.clone(), &_args.init_source, &[])?;

        let color_pass = SwapchainColorPass::new(cxt.clone(), &swapchain)?;

        let particles = Particles::builder()
            .cxt(cxt.clone())
            .frames_in_flight(&frames_in_flight)
            .swapchain(&swapchain)
            .render_pass(color_pass.renderpass())
            .kernel(kernel_compiler.shader())
            .init(init_compiler.shader())
            .build()
            .with_context(trace!("Unable to create particles!"))?;

        log::info!("{:#?}", particles);

        Ok(Self {
            start_time: Instant::now(),
            last_frame: Instant::now(),
            frames_in_flight,
            swapchain,
            swapchain_needs_rebuild: false,
            color_pass,
            particles,
            kernel_compiler,
            init_compiler,
            cxt,
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
                self.kernel_compiler.shader(),
                self.init_compiler.shader(),
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

        self.color_pass
            .begin_render_pass(&frame, [0.0, 0.0, 0.0, 0.0]);

        self.particles.draw(&frame)?;

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

impl LiveParticles {
    fn rebuild_swapchain(&mut self, window: &mut Window) -> Result<()> {
        self.swapchain_needs_rebuild = false;

        unsafe {
            // wait for all pending work to finish
            self.cxt.device_wait_idle()?;
        }

        self.swapchain = {
            let (w, h) = window.get_framebuffer_size();
            Swapchain::new(
                self.cxt.clone(),
                (w as u32, h as u32),
                Some(self.swapchain.raw()),
            )?
        };

        self.color_pass =
            SwapchainColorPass::new(self.cxt.clone(), &self.swapchain)?;

        self.particles
            .swapchain_rebuilt(&self.swapchain, self.color_pass.renderpass())?;

        Ok(())
    }
}

fn main() {
    app_main::<LiveParticles>();
}
