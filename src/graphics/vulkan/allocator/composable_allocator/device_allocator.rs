use {
    super::ComposableAllocator,
    crate::{
        graphics::vulkan::{allocator::AllocationRequirements, raii, Block},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// This allocator implementation directly allocates memory on the device.
pub struct DeviceAllocator {
    logical_device: Arc<raii::Device>,
}

impl DeviceAllocator {
    pub fn new(logical_device: Arc<raii::Device>) -> Self {
        Self { logical_device }
    }
}

impl ComposableAllocator for DeviceAllocator {
    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        // Allocate the underlying memory
        let memory = unsafe {
            self.logical_device
                .allocate_memory(&requirements.as_vk_allocate_info(), None)
                .with_context(trace!("Unable to allocate device memory!"))?
        };

        // Map the device memory if possible
        let mapped_ptr = if requirements
            .flags
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
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

        Ok(Block::new(
            0,
            requirements.allocation_size,
            memory,
            mapped_ptr,
        ))
    }

    fn free_memory(&mut self, block: &Block) {
        unsafe {
            self.logical_device.free_memory(block.memory(), None);
        }
    }

    fn owns(&self, _block: &Block) -> bool {
        true
    }
}
