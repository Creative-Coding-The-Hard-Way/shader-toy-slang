use {
    crate::trace,
    anyhow::{Context, Result},
    ash::vk,
};

/// Contains all of the information required to allocate a block of memory.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AllocationRequirements {
    pub alignment: u64,
    pub allocation_size: u64,
    pub memory_type_index: u32,
    pub flags: vk::MemoryPropertyFlags,
}

impl AllocationRequirements {
    /// Determines the allocation requirements based on system properties.
    pub fn new(
        properties: &vk::PhysicalDeviceMemoryProperties,
        requirements: &vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        let (memory_type_index, _) = properties
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
        Ok(Self {
            alignment: requirements.alignment,
            allocation_size: requirements.size,
            memory_type_index: memory_type_index as u32,
            flags,
        })
    }

    /// Constructs a compatible vkMemoryAllocateInfo struct.
    pub fn as_vk_allocate_info(&self) -> vk::MemoryAllocateInfo {
        vk::MemoryAllocateInfo {
            allocation_size: self.allocation_size,
            memory_type_index: self.memory_type_index,
            ..Default::default()
        }
    }
}
