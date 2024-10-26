use {
    crate::{
        graphics::vulkan::{raii, Device},
        trace,
    },
    anyhow::{bail, Context, Result},
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
    pub _memory: raii::DeviceMemory,
    aligned_unit_size: usize,
    count: usize,
    mapped_ptr: *mut std::ffi::c_void,
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

        // compute the total length of the buffer based on the aligned unit size
        // round up to the closest megabyte
        let buffer_size_in_bytes =
            (aligned_unit_size * count as u64).max(1024 * 1024);

        // create the buffer
        let buffer_create_info = vk::BufferCreateInfo {
            size: buffer_size_in_bytes,
            usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 1,
            p_queue_family_indices: &device.graphics_queue_family_index,
            ..Default::default()
        };
        let buffer = raii::Buffer::new(
            device.logical_device.clone(),
            &buffer_create_info,
        )?;

        // allocate the backing device memory
        let requirements =
            unsafe { device.get_buffer_memory_requirements(buffer.raw) };

        let memory_properties = unsafe {
            device
                .instance
                .ash
                .get_physical_device_memory_properties(device.physical_device)
        };
        let (memory_type_index, _) = memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                let type_bits = 1 << index;
                let is_supported_type =
                    type_bits & requirements.memory_type_bits != 0;
                let is_visible_and_coherent =
                    memory_type.property_flags.contains(
                        vk::MemoryPropertyFlags::HOST_VISIBLE
                            | vk::MemoryPropertyFlags::HOST_COHERENT,
                    );
                is_supported_type && is_visible_and_coherent
            })
            .with_context(trace!("Unable to find compatible memory type!"))?;

        let allocate_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index: memory_type_index as u32,
            ..Default::default()
        };
        let memory = raii::DeviceMemory::new(
            device.logical_device.clone(),
            &allocate_info,
        )?;

        let mapped_ptr = unsafe {
            device.map_memory(
                memory.raw,
                0,
                vk::WHOLE_SIZE,
                vk::MemoryMapFlags::empty(),
            )?
        };

        unsafe {
            device.bind_buffer_memory(buffer.raw, memory.raw, 0)?;
        };

        Ok(Self {
            buffer,
            count,
            aligned_unit_size: aligned_unit_size as usize,
            _memory: memory,
            mapped_ptr,
            _phantom_data: PhantomData::default(),
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
            self.mapped_ptr.byte_offset(offset) as *mut FrameDataT,
            data,
        );

        Ok(())
    }
}
