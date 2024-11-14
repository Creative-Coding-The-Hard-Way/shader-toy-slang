use {
    anyhow::Result,
    clap::Parser,
    demo_vk::{
        demo::{demo_main, Demo},
        graphics::{
            BindlessTextureAtlas, Recompiler, SwapchainColorPass, TextureLoader,
        },
    },
    std::{path::PathBuf, sync::Arc},
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
//   [[vk_binding(0, 1)]] ConstantBuffer<FrameData> frame;
//
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(C)]
pub struct FrameData {
    pub mouse_pos: [f32; 2],
    pub screen_size: [f32; 2],
    pub dt: f32,
    pub time: f32,
}

struct ShaderToySlang {
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

        Ok(Self {
            texture_atlas,
            color_pass,
            shader_compiler,
        })
    }

    fn draw(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut demo_vk::demo::Graphics<
            Self::Args,
        >,
        #[allow(unused_variables)] frame: &demo_vk::graphics::vulkan::Frame,
    ) -> Result<()> {
        self.color_pass
            .begin_render_pass(frame, [0.0, 0.0, 0.0, 0.0]);
        self.color_pass.end_render_pass(frame);
        Ok(())
    }
}

fn main() {
    demo_main::<ShaderToySlang>();
}
