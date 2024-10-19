use {
    anyhow::{bail, Context, Result},
    ash::vk,
    sts::{
        graphics::vulkan::{raii, Device},
        trace,
    },
};

/// A CPU accessible buffer with some convenience functions for uploading data.
#[derive(Debug)]
pub struct CPUBuffer {
    pub size: usize,
    pub buffer: raii::Buffer,
    pub _memory: raii::DeviceMemory,
    mapped_ptr: *mut std::ffi::c_void,
}

impl CPUBuffer {
    pub fn allocate(device: &Device, size: u64) -> Result<Self> {
        let buffer_create_info = vk::BufferCreateInfo {
            size,
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
            size: size as usize,
            _memory: memory,
            mapped_ptr,
        })
    }

    pub fn write<D: Copy>(&mut self, data: D) -> Result<()> {
        if std::mem::size_of_val(&data) > self.size {
            bail!(trace!("Attempted to overwrite gpu buffer!")());
        }

        unsafe { std::ptr::write_volatile(self.mapped_ptr as *mut D, data) };

        Ok(())
    }
}
