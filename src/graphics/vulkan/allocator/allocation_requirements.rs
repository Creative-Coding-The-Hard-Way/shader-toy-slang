use {
    crate::{graphics::vulkan::allocator::HumanizedSize, trace},
    anyhow::{Context, Result},
    ash::vk,
};

/// Contains all of the information required to allocate a block of memory.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AllocationRequirements {
    pub alignment: u64,
    pub allocation_size: u64,
    pub memory_type_index: u32,
    pub flags: vk::MemoryPropertyFlags,
    pub should_be_dedicated: bool,
}

impl AllocationRequirements {
    /// Determines the allocation requirements based on system properties.
    pub fn new(
        properties: &vk::PhysicalDeviceMemoryProperties,
        requirements: &vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
        dedicated: bool,
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
            should_be_dedicated: dedicated,
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

impl std::fmt::Debug for AllocationRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AllocationRequirements")
            .field("alignment", &self.alignment)
            .field("allocation_size", &HumanizedSize(self.allocation_size))
            .field("memory_type_index", &self.memory_type_index)
            .field("flags", &self.flags)
            .field("should_be_dedicated", &self.should_be_dedicated)
            .finish()
    }
}
