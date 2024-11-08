use {
    anyhow::{Context, Result},
    clap::Parser,
    glfw::{Action, Key, Window, WindowEvent},
    std::sync::Arc,
    sts::{
        app::{app_main, App},
        graphics::{
            ortho_projection,
            vulkan::{
                FrameStatus, FramesInFlight, PresentImageStatus, Swapchain,
                VulkanContext,
            },
            Sprite, SpriteLayer, StreamingSprites, SwapchainColorPass,
        },
        trace,
    },
};

#[derive(Parser, Debug)]
struct Args {}

/// A sprites demo.
struct Sprites {
    world_layer: SpriteLayer,
    sprites: StreamingSprites,

    // Vulkan resources
    frames_in_flight: FramesInFlight,
    color_pass: SwapchainColorPass,
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

        let color_pass = SwapchainColorPass::new(ctx.clone(), &swapchain)?;

        let world_layer = SpriteLayer::builder()
            .ctx(ctx.clone())
            .frames_in_flight(&frames_in_flight)
            .render_pass(color_pass.renderpass())
            .swapchain(&swapchain)
            .projection(ortho_projection(w as f32 / h as f32, 10.0))
            .build()?;

        let sprites = StreamingSprites::new(ctx.clone(), &frames_in_flight)?;

        Ok(Self {
            world_layer,
            sprites,
            frames_in_flight,
            color_pass,
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
                self.color_pass.renderpass(),
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

        self.color_pass
            .begin_render_pass(&frame, [0.0, 0.0, 0.0, 0.0]);

        self.sprites
            .add(Sprite {
                pos: [0.0, 0.0],
                ..Default::default()
            })
            .add(Sprite {
                pos: [3.0, 0.0],
                size: [0.5, 1.0],
                tint: [0.2, 0.5, 0.2, 1.0],
                ..Default::default()
            })
            .flush(&frame)?;

        self.world_layer
            .begin_frame_commands(&frame)?
            .draw(&self.sprites)?
            .finish();

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

        self.color_pass =
            SwapchainColorPass::new(self.ctx.clone(), &self.swapchain)?;

        Ok(())
    }
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
