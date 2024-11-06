use {
    crate::{
        graphics::vulkan::{raii, Device, Frame, FramesInFlight, OwnedBlock},
        trace,
    },
    anyhow::{bail, Result},
    ash::vk,
    std::marker::PhantomData,
};

/// A CPU accessible buffer with some convenience functions for uploading
/// per-frame data.
#[derive(Debug)]
pub struct UniformBuffer<DataT: Sized + Copy> {
    buffer: raii::Buffer,
    block: OwnedBlock,
    aligned_unit_size: usize,
    count: usize,
    _phantom_data: PhantomData<DataT>,
}

impl<DataT> UniformBuffer<DataT>
where
    DataT: Sized + Copy,
{
    /// Allocates a buffer with enough space for count copies of DataT aligned
    /// such that each copy can be bound to a separate descriptor set.
    pub fn allocate(device: &Device, count: usize) -> Result<Self> {
        // compute the aligned size for each element in the buffer
        let properties = unsafe {
            device
                .instance
                .ash
                .get_physical_device_properties(device.physical_device)
        };
        let aligned_unit_size: u64 = {
            let count = size_of::<DataT>() as u64
                / properties.limits.min_uniform_buffer_offset_alignment;
            (count + 1) * properties.limits.min_uniform_buffer_offset_alignment
        };

        let buffer_size_in_bytes = aligned_unit_size * count as u64;

        let (block, buffer) = OwnedBlock::allocate_buffer(
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

    /// Allocates a new buffer and GPU memory for holding per-frame uniform
    /// data.
    pub fn allocate_per_frame(
        device: &Device,
        frames_in_flight: &FramesInFlight,
    ) -> Result<Self> {
        Self::allocate(device, frames_in_flight.frame_count())
    }

    /// Returns a non-owning copy of the Vulkan buffer handle.
    pub fn buffer(&self) -> vk::Buffer {
        self.buffer.raw
    }

    /// Updates GPU memory with the provided data for the current frame.
    pub fn update_frame_data(
        &mut self,
        frame: &Frame,
        data: DataT,
    ) -> Result<()> {
        // SAFE: because borrowing the Frame means that no pending graphics
        // commands can still reference the targeted region of the
        // buffer.
        unsafe { self.write_indexed(frame.frame_index(), data) }
    }

    /// Returns the byte-offset into the buffer for the corresponding Frame's
    /// data.
    pub fn offset_for_index(&self, frame_index: usize) -> u64 {
        (frame_index * self.aligned_unit_size) as u64
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
        data: DataT,
    ) -> Result<()> {
        if index >= self.count {
            bail!(
                trace!("Attempt to write to index {}/{}", index, self.count)()
            );
        }

        let offset = self.offset_for_index(index) as isize;
        std::ptr::copy_nonoverlapping(
            &data,
            self.block.mapped_ptr().byte_offset(offset) as *mut DataT,
            1,
        );

        Ok(())
    }
}
