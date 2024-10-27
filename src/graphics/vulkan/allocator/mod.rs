pub mod block;
pub mod owned_block;

use {
    crate::{
        graphics::vulkan::{raii, Block},
        trace,
    },
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

    /// Allocates device memory according to the given requirements.
    pub fn allocate_memory(
        &self,
        requirements: &vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<Block> {
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
        let memory = unsafe {
            self.logical_device
                .allocate_memory(&allocate_info, None)
                .with_context(trace!("Unable to allocate device memory!"))?
        };
        let mapped_ptr =
            if flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
                unsafe {
                    self.logical_device
                        .map_memory(
                            memory,
                            0,
                            vk::WHOLE_SIZE,
                            vk::MemoryMapFlags::empty(),
                        )
                        .with_context(trace!("Unable to map memory!"))?
                }
            } else {
                std::ptr::null_mut()
            };
        Ok(Block::new(0, requirements.size, memory, mapped_ptr))
    }

    /// Free the allocated block.
    pub fn free(&self, block: &Block) {
        unsafe {
            self.logical_device.free_memory(block.memory(), None);
        }
    }
}

impl std::fmt::Debug for Allocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Allocator").finish_non_exhaustive()
    }
}
