use {
    crate::{
        graphics::vulkan::{raii, OwnedBlock, VulkanContext},
        trace,
    },
    anyhow::{bail, Result},
    ash::vk,
    std::marker::PhantomData,
};

/// A CPU accessible buffer with some convenience functions for uploading data.
#[derive(Debug)]
pub struct CPUBuffer<DataT: Sized + Copy> {
    buffer: raii::Buffer,
    block: OwnedBlock,
    count: usize,
    _phantom_data: PhantomData<DataT>,
}

impl<DataT> CPUBuffer<DataT>
where
    DataT: Sized + Copy,
{
    /// Allocates a new buffer and GPU memory for holding data.
    ///
    /// Total size is count * size_of<DataT>()
    pub fn allocate(
        cxt: &VulkanContext,
        count: usize,
        usage: vk::BufferUsageFlags,
    ) -> Result<Self> {
        let buffer_size_in_bytes = (count * size_of::<DataT>()) as u64;

        let (block, buffer) = OwnedBlock::allocate_buffer(
            cxt.allocator.clone(),
            &vk::BufferCreateInfo {
                size: buffer_size_in_bytes,
                usage,
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
            count,
            _phantom_data: PhantomData,
        })
    }

    /// Returns a non-owning copy of the Vulkan buffer handle.
    pub fn buffer(&self) -> vk::Buffer {
        self.buffer.raw
    }

    /// The size of the buffer in bytes.
    pub fn size_in_bytes(&self) -> u64 {
        (self.count * size_of::<DataT>()) as u64
    }

    /// The maximum number of items that can be saved in this buffer.
    pub fn capacity(&self) -> usize {
        self.count
    }

    /// Writes data into the GPU memory at the given index.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - the caller must synchronize access to the region being written.
    pub unsafe fn write_data(
        &mut self,
        start_index: usize,
        data: &[DataT],
    ) -> Result<()> {
        if start_index + data.len() > self.count {
            bail!(trace!(
                "Out of bounds write attempted! {}/{}",
                start_index + data.len(),
                self.count
            )());
        }

        std::ptr::copy_nonoverlapping(
            data.as_ptr(),
            (self.block.mapped_ptr() as *mut DataT).add(start_index),
            data.len(),
        );

        Ok(())
    }
}
