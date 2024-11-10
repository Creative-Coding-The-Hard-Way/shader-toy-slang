use {
    anyhow::{Context, Result},
    clap::Parser,
    core::f32,
    glfw::{Action, Key, Window, WindowEvent},
    nalgebra::Similarity2,
    std::{sync::Arc, time::Instant},
    sts::{
        app::{app_main, App},
        graphics::{
            ortho_projection,
            vulkan::{
                FrameStatus, FramesInFlight, PresentImageStatus, Swapchain,
                VulkanContext,
            },
            BindlessTextureAtlas, Sprite, SpriteLayer, StreamingSprites,
            SwapchainColorPass, TextureLoader,
        },
        trace,
    },
};

#[derive(Parser, Debug)]
struct Args {}

/// A sprites demo.
struct Sprites {
    start_time: Instant,
    last_frame: Instant,

    world_layer: SpriteLayer,
    sprites: StreamingSprites,
    atlas: BindlessTextureAtlas,

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

        let mut atlas = BindlessTextureAtlas::new(
            ctx.clone(),
            1024 * 10,
            &frames_in_flight,
        )?;

        let mut loader = TextureLoader::new(ctx.clone())?;
        let sprite_texture =
            Arc::new(loader.load_from_file("./examples/sprites/sprite.jpg")?);

        atlas.add_texture(sprite_texture);

        let world_layer = SpriteLayer::builder()
            .ctx(ctx.clone())
            .frames_in_flight(&frames_in_flight)
            .texture_atlas_layout(atlas.descriptor_set_layout())
            .render_pass(color_pass.renderpass())
            .swapchain(&swapchain)
            .projection(ortho_projection(w as f32 / h as f32, 10.0))
            .build()?;

        let sprites = StreamingSprites::new(ctx.clone(), &frames_in_flight)?;

        Ok(Self {
            start_time: Instant::now(),
            last_frame: Instant::now(),
            world_layer,
            sprites,
            atlas,
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

        let (_dt, t) = {
            let now = Instant::now();
            let dt = now.duration_since(self.last_frame).as_secs_f32();
            self.last_frame = now;
            (dt, now.duration_since(self.start_time).as_secs_f32())
        };

        let max = 1_000;
        for i in 0..max {
            let angle = t + f32::consts::TAU * i as f32 / max as f32;
            self.sprites.add(
                Sprite::new()
                    .with_texture(0)
                    .with_sampler(0)
                    .with_similarity(&Similarity2::new(
                        [4.0 * angle.cos(), 4.0 * (angle * 3.1).sin()].into(),
                        0.0,
                        0.05,
                    )),
            );
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

        self.atlas.bind_frame_descriptor(&frame)?;
        self.sprites.flush(&frame)?;
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

        // wait for all pending work to finish
        self.frames_in_flight.wait_for_all_frames_to_complete()?;

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
