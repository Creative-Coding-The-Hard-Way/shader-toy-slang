use {
    crate::{
        graphics::vulkan::{raii, Device, OwnedBlock},
        trace,
    },
    anyhow::{bail, Result},
    ash::vk,
    std::marker::PhantomData,
};

/// A CPU accessible buffer with some convenience functions for uploading data.
///
/// # How It Works
///
/// The UniformBuffer allocates enough data to hold N copies of the FrameDataT
/// data type. This allows up to N independent frames-in-flight that can
/// independently update their data.
///
/// Notably, the implementation ensures that each copy of the frame data is
/// aligned to the devices min uniform buffer offset alignment.
///
/// # Performance
///
/// This implementation always uses a dedicated host-coherent allocation for
/// storing per-frame uniform data. This is fine for an application that's
/// *only* presenting a single uniform buffer, but will need to be changed if
/// used as part of a larger application.
#[derive(Debug)]
pub struct UniformBuffer<FrameDataT: Sized + Copy + Default> {
    pub buffer: raii::Buffer,
    pub block: OwnedBlock,
    aligned_unit_size: usize,
    count: usize,
    _phantom_data: PhantomData<FrameDataT>,
}

impl<FrameDataT> UniformBuffer<FrameDataT>
where
    FrameDataT: Sized + Copy + Default,
{
    /// Allocate a new buffer and GPU memory for holding per-frame uniform data.
    ///
    /// The buffer will have enough size for `count` copies of the frame data.
    pub fn allocate(device: &Device, count: usize) -> Result<Self> {
        // compute the aligned size for each element in the buffer
        let properties = unsafe {
            device
                .instance
                .ash
                .get_physical_device_properties(device.physical_device)
        };
        let aligned_unit_size: u64 = {
            let count = std::mem::size_of::<FrameDataT>() as u64
                / properties.limits.min_uniform_buffer_offset_alignment;
            (count + 1) * properties.limits.min_uniform_buffer_offset_alignment
        };
        log::trace!("Unit size: {}", aligned_unit_size);

        let buffer_size_in_bytes = aligned_unit_size * count as u64;

        // create the buffer
        let (buffer, block) = OwnedBlock::allocate_buffer(
            device.allocator.clone(),
            &vk::BufferCreateInfo {
                size: buffer_size_in_bytes,
                usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &device.graphics_queue_family_index,
                ..Default::default()
            },
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        Ok(Self {
            buffer,
            block,
            count,
            aligned_unit_size: aligned_unit_size as usize,
            _phantom_data: PhantomData,
        })
    }

    /// Returns the byte offset into the buffer for the item at an index.
    pub fn offset_for_index(&self, index: usize) -> u64 {
        (index * self.aligned_unit_size) as u64
    }

    /// Writes data into the GPU memory at the given index.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - the caller must synchronize access to the region being written.
    pub unsafe fn write_indexed(
        &mut self,
        index: usize,
        data: FrameDataT,
    ) -> Result<()> {
        if index >= self.count {
            bail!(
                trace!("Attempt to write to index {}/{}", index, self.count)()
            );
        }

        let offset = self.offset_for_index(index) as isize;
        std::ptr::write_volatile(
            self.block.mapped_ptr().byte_offset(offset) as *mut FrameDataT,
            data,
        );

        Ok(())
    }
}
