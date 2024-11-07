use {
    crate::{
        graphics::vulkan::{raii, OwnedBlock, VulkanContext},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// A CPU-Writable buffer for use when uploading image data to DEVICE_LOCAL
/// memory.
pub struct TransferBuffer {
    buffer: raii::Buffer,
    block: OwnedBlock,
    capacity: u64,
    cxt: Arc<VulkanContext>,
}

impl TransferBuffer {
    /// Creates a new transfer buffer with an initial capacity specified in
    /// bytes.
    pub fn new(cxt: Arc<VulkanContext>, capacity: u64) -> Result<Self> {
        let (block, buffer) = OwnedBlock::allocate_buffer(
            cxt.allocator.clone(),
            &vk::BufferCreateInfo {
                size: capacity,
                usage: vk::BufferUsageFlags::TRANSFER_SRC,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &cxt.graphics_queue_family_index,
                ..Default::default()
            },
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;
        Ok(Self {
            buffer,
            block,
            capacity,
            cxt,
        })
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
        if self.needs_realloc(data.len() as u64) {
            self.realloc(data.len() as u64)?;
        }

        // safe because the block has been realloced to have enough capacity
        // for data() bytes.
        std::ptr::copy_nonoverlapping(
            data.as_ptr(),
            self.block.mapped_ptr() as *mut u8,
            data.len(),
        );

        Ok(())
    }

    /// A non-owning copy of the Vulkan buffer handle.
    pub fn buffer(&self) -> vk::Buffer {
        self.buffer.raw
    }

    // Private API ------------------------------------------------------------

    /// Returns true if the internal buffer needs to be resized.
    fn needs_realloc(&self, required_capacity: u64) -> bool {
        required_capacity >= self.capacity
    }

    /// Reallocates the backing buffer and memory.
    ///
    /// If there is already enough capacity for the required capacity, then this
    /// function is a no-op.
    fn realloc(&mut self, required_capacity: u64) -> Result<()> {
        let mut target_capacity = self.capacity;
        while target_capacity < required_capacity {
            target_capacity *= 2;
        }

        (self.block, self.buffer) = OwnedBlock::allocate_buffer(
            self.cxt.allocator.clone(),
            &vk::BufferCreateInfo {
                size: target_capacity,
                usage: vk::BufferUsageFlags::TRANSFER_SRC,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &self.cxt.graphics_queue_family_index,
                ..Default::default()
            },
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT,
        )
        .with_context(trace!("Unable to resize the transfer buffer!"))?;

        self.capacity = target_capacity;

        Ok(())
    }
}
