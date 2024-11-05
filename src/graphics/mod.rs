mod fullscreen_quad;
mod particles;
mod recompiler;
mod texture_loader;
pub mod vulkan;

use nalgebra::Matrix4;

pub use self::{
    fullscreen_quad::FullscreenQuad,
    particles::Particles,
    recompiler::Recompiler,
    texture_loader::{Texture, TextureLoader},
};

pub fn ortho_projection(aspect: f32, height: f32) -> Matrix4<f32> {
    let w = height * aspect;
    let h = height;
    #[rustfmt::skip]
    let projection = Matrix4::new(
        2.0 / w,  0.0,     0.0, 0.0,
        0.0,     -2.0 / h, 0.0, 0.0,
        0.0,      0.0,     1.0, 0.0,
        0.0,      0.0,     0.0, 1.0,
    );
    projection
}
