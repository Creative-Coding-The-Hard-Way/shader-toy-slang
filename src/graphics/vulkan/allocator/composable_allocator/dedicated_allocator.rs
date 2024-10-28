use {
    crate::graphics::vulkan::{
        allocator::{
            allocation_requirements::AllocationRequirements,
            ComposableAllocator,
        },
        Block,
    },
    anyhow::{bail, Result},
};

/// The Dedicated Allocator is responsible for allocating exclusive chunks of
/// memory from the device.
///
/// Dedicated allocations should typically only be for large blocks that get
/// subdivided.
pub struct DedicatedAllocator<A: ComposableAllocator> {
    allocations: Vec<Block>,
    device_allocator: A,
}

impl<A: ComposableAllocator> DedicatedAllocator<A> {
    pub fn new(device_allocator: A) -> Self {
        Self {
            allocations: Vec::new(),
            device_allocator,
        }
    }
}

impl<A: ComposableAllocator> ComposableAllocator for DedicatedAllocator<A> {
    fn owns(&self, block: &Block) -> bool {
        self.allocations
            .iter()
            .any(|owned_block| owned_block.memory() == block.memory())
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        if !requirements.should_be_dedicated {
            bail!("Not a dedicated allocation.")
        }
        let block = self.device_allocator.allocate_memory(requirements)?;
        self.allocations.push(block);
        Ok(block)
    }

    fn free_memory(&mut self, block: &Block) {
        if let Some((index, _)) = self
            .allocations
            .iter()
            .enumerate()
            .find(|(_, owned_block)| owned_block.memory() == block.memory())
        {
            self.allocations.swap_remove(index);
            self.device_allocator.free_memory(block);
        }
    }
}
