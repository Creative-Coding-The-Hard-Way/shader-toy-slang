mod streaming_sprites;

use ash::vk;

pub use self::streaming_sprites::StreamingSprites;

/// A batch of sprites in device memory.
pub trait SpriteBatch {
    /// The backing device buffer.
    fn buffer(&self) -> vk::Buffer;

    /// The number of sprites to render.
    fn count(&self) -> u32;
}

impl SpriteBatch for (u32, vk::Buffer) {
    fn buffer(&self) -> vk::Buffer {
        self.1
    }

    fn count(&self) -> u32 {
        self.0
    }
}

impl SpriteBatch for (vk::Buffer, u32) {
    fn buffer(&self) -> vk::Buffer {
        self.0
    }

    fn count(&self) -> u32 {
        self.1
    }
}
