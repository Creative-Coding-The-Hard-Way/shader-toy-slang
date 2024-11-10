use nalgebra::{Affine2, Similarity2};

/// A sprite is a textured quad that can be rendered with a SpriteLayer.
///
/// Sprites can be textured using texture indices provided by the currently
/// bound texture atlas. Sprites can be translated, rotated, scaled, and skewed
/// to achieve the desired shape.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Sprite {
    transform: [f32; 6],
    uv_pos: [f32; 2],
    uv_size: [f32; 2],
    tint: [f32; 4],
    texture: i32,
    sampler: u32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            transform: [
                1.0, 0.0, 0.0, // row 1
                0.0, 1.0, 0.0, // row 2
            ],
            uv_pos: [0.0, 0.0],
            uv_size: [1.0, 1.0],
            tint: [1.0, 1.0, 1.0, 1.0],
            texture: -1,
            sampler: 0,
        }
    }
}

impl Sprite {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_tint(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.tint = [r, g, b, a];
        self
    }

    pub fn with_uv(mut self, uv_pos: [f32; 2], uv_size: [f32; 2]) -> Self {
        self.uv_pos = uv_pos;
        self.uv_size = uv_size;
        self
    }

    pub fn with_sampler(mut self, sampler: u32) -> Self {
        self.sampler = sampler;
        self
    }

    pub fn with_texture(mut self, texture: i32) -> Self {
        self.texture = texture;
        self
    }

    pub fn with_similarity(self, similarity: &Similarity2<f32>) -> Self {
        self.with_transform(&nalgebra::convert(*similarity))
    }

    pub fn with_transform(mut self, transform: &Affine2<f32>) -> Self {
        self.transform = [
            transform[(0, 0)],
            transform[(1, 0)],
            transform[(0, 1)],
            transform[(1, 1)],
            transform[(0, 2)],
            transform[(1, 2)],
        ];
        self
    }
}
