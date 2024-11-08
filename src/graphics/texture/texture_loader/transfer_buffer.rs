use {
    crate::graphics::vulkan::{CPUBuffer, VulkanContext},
    anyhow::Result,
    ash::vk,
    std::sync::Arc,
};

/// A CPU-Writable buffer for use when uploading image data to DEVICE_LOCAL
/// memory.
pub struct TransferBuffer {
    buffer: CPUBuffer<u8>,
    cxt: Arc<VulkanContext>,
}

impl TransferBuffer {
    /// Creates a new transfer buffer with an initial capacity specified in
    /// bytes.
    pub fn new(cxt: Arc<VulkanContext>) -> Result<Self> {
        let buffer = CPUBuffer::allocate(
            &cxt,
            1024 * 1024, // initial capacity of 1mb
            vk::BufferUsageFlags::TRANSFER_SRC,
        )?;
        Ok(Self { buffer, cxt })
    }

    /// Uploads the provided bytes to GPU memory.
    ///
    /// The internal buffer and memory will be resized as needed.
    ///
    /// ## Safety
    ///
    /// Unsafe because the caller must ensure the transfer buffer is not in use
    /// by the GPU when it is updated.
    pub unsafe fn upload_bytes(&mut self, data: &[u8]) -> Result<()> {
        if self.needs_realloc(data.len()) {
            self.realloc(data.len())?;
        }
        self.buffer.write_data(0, data)
    }

    /// A non-owning copy of the Vulkan buffer handle.
    pub fn buffer(&self) -> vk::Buffer {
        self.buffer.buffer()
    }

    // Private API ------------------------------------------------------------

    /// Returns true if the internal buffer needs to be resized.
    fn needs_realloc(&self, required_capacity: usize) -> bool {
        required_capacity >= self.buffer.capacity()
    }

    /// Reallocates the backing buffer and memory.
    fn realloc(&mut self, required_capacity: usize) -> Result<()> {
        let mut target_capacity = self.buffer.capacity();
        while target_capacity < required_capacity {
            target_capacity *= 2;
        }

        self.buffer = CPUBuffer::allocate(
            &self.cxt,
            target_capacity,
            vk::BufferUsageFlags::TRANSFER_SRC,
        )?;

        log::info!("transfer buffer reallocated: {:#?}", self.buffer);

        Ok(())
    }
}
