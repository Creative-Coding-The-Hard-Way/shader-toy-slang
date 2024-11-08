use {
    crate::graphics::{
        vulkan::{CPUBuffer, Frame, FramesInFlight, VulkanContext},
        Sprite,
    },
    anyhow::Result,
    ash::vk,
    std::sync::Arc,
};

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

/// A sprite-batch implementation that supports streaming new sprites every
/// frame.
pub struct StreamingSprites {
    current_frame_index: usize,
    sprites: Vec<Sprite>,
    buffers: Vec<CPUBuffer<Sprite>>,
    sprite_counts: Vec<u32>,
    ctx: Arc<VulkanContext>,
}

impl SpriteBatch for StreamingSprites {
    fn buffer(&self) -> vk::Buffer {
        self.buffers[self.current_frame_index].buffer()
    }

    fn count(&self) -> u32 {
        self.sprite_counts[self.current_frame_index]
    }
}

impl StreamingSprites {
    pub fn new(
        ctx: Arc<VulkanContext>,
        frames_in_flight: &FramesInFlight,
    ) -> Result<Self> {
        let mut buffers = vec![];
        for _ in 0..frames_in_flight.frame_count() {
            buffers.push(CPUBuffer::allocate(
                &ctx,
                1,
                vk::BufferUsageFlags::STORAGE_BUFFER,
            )?);
        }
        Ok(Self {
            current_frame_index: 0,
            sprites: vec![],
            buffers,
            sprite_counts: vec![0; frames_in_flight.frame_count()],
            ctx,
        })
    }

    /// Add a sprite to be rendered by the next call to flush().
    pub fn add(&mut self, sprite: Sprite) -> &mut Self {
        self.sprites.push(sprite);
        self
    }

    /// Flush all current sprites into device memory for rendering in the frame.
    pub fn flush(&mut self, frame: &Frame) -> Result<()> {
        if self.buffers[frame.frame_index()].capacity() < self.sprites.len() {
            self.buffers[frame.frame_index()] = CPUBuffer::allocate(
                &self.ctx,
                self.sprites.len() * 2,
                vk::BufferUsageFlags::STORAGE_BUFFER,
            )?;
        }

        unsafe {
            // SAFE: because access to the buffer is synchronized by the frame
            self.buffers[frame.frame_index()].write_data(0, &self.sprites)?
        }
        self.sprite_counts[frame.frame_index()] = self.sprites.len() as u32;
        self.sprites.clear();

        self.current_frame_index = frame.frame_index();

        Ok(())
    }
}
