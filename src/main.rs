mod fullscreen_effect;

use {
    anyhow::Result,
    clap::Parser,
    demo_vk::{
        demo::{demo_main, Demo, Graphics},
        graphics::{
            vulkan::Frame, BindlessTextureAtlas, Recompiler,
            SwapchainColorPass, TextureLoader,
        },
    },
    fullscreen_effect::{FrameData, FullscreenEffect},
    std::{path::PathBuf, sync::Arc, time::Instant},
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

struct ShaderToySlang {
    last_frame: Instant,
    start_time: Instant,

    effect: FullscreenEffect,
    texture_atlas: BindlessTextureAtlas,
    shader_compiler: Recompiler,
    color_pass: SwapchainColorPass,
}

impl Demo for ShaderToySlang {
    type Args = Args;

    fn new(
        window: &mut glfw::Window,
        gfx: &mut demo_vk::demo::Graphics<Self::Args>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_title(
            gfx.args
                .frag_shader
                .parent()
                .and_then(|a| a.to_str())
                .unwrap_or("shader-toy-slang"),
        );

        let color_pass =
            SwapchainColorPass::new(gfx.vulkan.clone(), &gfx.swapchain)?;
        let shader_compiler = Recompiler::new(
            gfx.vulkan.clone(),
            &gfx.args.frag_shader,
            &gfx.args.additional_watch_dir,
        )?;

        let texture_atlas = {
            let mut texture_atlas = BindlessTextureAtlas::new(
                gfx.vulkan.clone(),
                64,
                &gfx.frames_in_flight,
            )?;
            let mut loader = TextureLoader::new(gfx.vulkan.clone())?;
            for path in &gfx.args.texture {
                let texture = loader.load_from_file(path)?;
                texture_atlas.add_texture(Arc::new(texture));
            }
            texture_atlas
        };

        let effect = FullscreenEffect::builder()
            .ctx(gfx.vulkan.clone())
            .frames_in_flight(&gfx.frames_in_flight)
            .texture_atlas_layout(texture_atlas.descriptor_set_layout())
            .render_pass(color_pass.renderpass())
            .effect_shader(shader_compiler.shader())
            .build()?;

        Ok(Self {
            start_time: Instant::now(),
            last_frame: Instant::now(),
            effect,
            texture_atlas,
            color_pass,
            shader_compiler,
        })
    }

    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        if self.shader_compiler.check_for_update()? {
            gfx.frames_in_flight.wait_for_all_frames_to_complete()?;
            self.effect.rebuild_pipeline(
                self.color_pass.renderpass(),
                Some(self.shader_compiler.shader()),
            )?;
        }
        Ok(())
    }

    fn draw(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
        #[allow(unused_variables)] frame: &Frame,
    ) -> Result<()> {
        self.color_pass
            .begin_render_pass(frame, [0.0, 0.0, 0.0, 0.0]);
        self.texture_atlas.bind_frame_descriptor(frame)?;

        unsafe {
            gfx.vulkan.cmd_set_viewport(
                frame.command_buffer(),
                0,
                &[gfx.swapchain.viewport()],
            );
            gfx.vulkan.cmd_set_scissor(
                frame.command_buffer(),
                0,
                &[gfx.swapchain.scissor()],
            );
        }

        let (mx, my) = window.get_cursor_pos();
        let (sx, sy) = window.get_size();
        let (w, h) = (sx as f32, sy as f32);

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32();
        let time = now.duration_since(self.start_time).as_secs_f32();
        self.last_frame = now;

        self.effect.draw(
            frame,
            FrameData {
                mouse_pos: [
                    (mx as f32 / w) * 2.0 - 1.0,
                    1.0 - 2.0 * (my as f32 / h),
                ],
                screen_size: [w, h],
                dt,
                time,
            },
        )?;

        self.color_pass.end_render_pass(frame);
        Ok(())
    }

    fn rebuild_swapchain_resources(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Args>,
    ) -> Result<()> {
        self.color_pass =
            SwapchainColorPass::new(gfx.vulkan.clone(), &gfx.swapchain)?;
        self.effect
            .rebuild_pipeline(self.color_pass.renderpass(), None)?;
        Ok(())
    }

    fn unpaused(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Args>,
    ) -> Result<()> {
        self.last_frame = Instant::now();
        Ok(())
    }
}

fn main() {
    demo_main::<ShaderToySlang>();
}
