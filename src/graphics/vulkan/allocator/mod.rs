use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// A Vulkan device memory allocator.
///
/// # Ownership
///
/// The owner of the Allocator is responsible for ensuring that the phisacal
/// device, the ash library instance, and the Vulkan logical device all outlive
/// the Allocator.
pub struct Allocator {
    // Non-owning copies of Vulkan resources.
    logical_device: Arc<raii::Device>,

    memory_properties: vk::PhysicalDeviceMemoryProperties,
}

impl Allocator {
    pub fn new(
        logical_device: Arc<raii::Device>,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let memory_properties = unsafe {
            logical_device
                .instance
                .get_physical_device_memory_properties(physical_device)
        };
        Ok(Self {
            logical_device,
            memory_properties,
        })
    }

    /// Creates an image and allocates memory to back it.
    ///
    /// The image is bound to the memory prior to return, so the caller can use
    /// it right away.
    pub fn allocate_image(
        &self,
        image_create_info: &vk::ImageCreateInfo,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<(raii::Image, raii::DeviceMemory)> {
        let image =
            raii::Image::new(self.logical_device.clone(), image_create_info)
                .with_context(trace!("Unable to create image!"))?;

        let requirements = {
            let mut dedicated = vk::MemoryDedicatedRequirements::default();
            let requirements = unsafe {
                let mut out = vk::MemoryRequirements2::default()
                    .push_next(&mut dedicated);

                self.logical_device.get_image_memory_requirements2(
                    &vk::ImageMemoryRequirementsInfo2 {
                        image: image.raw,
                        ..Default::default()
                    },
                    &mut out,
                );

                out.memory_requirements
            };
            requirements
        };

        let memory = self
            .allocate_memory(&requirements, flags)
            .with_context(trace!("Unable to allocate memory for image!"))?;

        unsafe {
            self.logical_device
                .bind_image_memory(image.raw, memory.raw, 0)
                .with_context(trace!("Unable to bind image memory!"))?;
        };

        Ok((image, memory))
    }

    /// Creates a buffer and allocates memory to back it.
    ///
    /// The buffer is bound to the memory prior to return, so the caller can use
    /// it right away.
    pub fn allocate_buffer(
        &self,
        buffer_create_info: &vk::BufferCreateInfo,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<(raii::Buffer, raii::DeviceMemory)> {
        let buffer =
            raii::Buffer::new(self.logical_device.clone(), buffer_create_info)
                .with_context(trace!("Unable to create buffer!"))?;

        let requirements = {
            let mut dedicated = vk::MemoryDedicatedRequirements::default();
            let requirements = unsafe {
                let mut out = vk::MemoryRequirements2::default()
                    .push_next(&mut dedicated);

                self.logical_device.get_buffer_memory_requirements2(
                    &vk::BufferMemoryRequirementsInfo2 {
                        buffer: buffer.raw,
                        ..Default::default()
                    },
                    &mut out,
                );

                out.memory_requirements
            };
            requirements
        };

        let memory = self
            .allocate_memory(&requirements, flags)
            .with_context(trace!("Unable to allocate memory for buffer!"))?;

        unsafe {
            self.logical_device
                .bind_buffer_memory(buffer.raw, memory.raw, 0)
                .with_context(trace!("Unable to bind buffer to memory!"))?;
        };

        Ok((buffer, memory))
    }

    /// Allocates device memory according to the given requirements.
    pub fn allocate_memory(
        &self,
        requirements: &vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<raii::DeviceMemory> {
        let (memory_type_index, _) = self
            .memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                let type_bits = 1 << index;
                let is_supported_type =
                    type_bits & requirements.memory_type_bits != 0;
                let is_visible_and_coherent =
                    memory_type.property_flags.contains(flags);
                is_supported_type && is_visible_and_coherent
            })
            .with_context(trace!("Unable to find compatible memory type!"))?;

        let allocate_info = vk::MemoryAllocateInfo {
            allocation_size: requirements.size,
            memory_type_index: memory_type_index as u32,
            ..Default::default()
        };
        raii::DeviceMemory::new(self.logical_device.clone(), &allocate_info)
    }
}
