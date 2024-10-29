use {
    crate::graphics::vulkan::{
        allocator::{
            allocation_requirements::AllocationRequirements,
            ComposableAllocator,
        },
        Block,
    },
    anyhow::Result,
};

/// An allocator that rounds any allocation below a threshold up to the
/// threshold value.
pub struct RoundUpAllocator<A: ComposableAllocator> {
    allocator: A,
    threshold: u64,
}

impl<A: ComposableAllocator> RoundUpAllocator<A> {
    #[allow(unused)]
    pub fn new(threshold: u64, allocator: A) -> Self {
        Self {
            allocator,
            threshold,
        }
    }
}

impl<A: ComposableAllocator> ComposableAllocator for RoundUpAllocator<A> {
    fn owns(&self, block: &Block) -> bool {
        self.allocator.owns(block)
    }

    fn allocate_memory(
        &mut self,
        mut requirements: AllocationRequirements,
    ) -> Result<Block> {
        requirements.allocation_size =
            requirements.allocation_size.max(self.threshold);
        self.allocator.allocate_memory(requirements)
    }

    fn free_memory(&mut self, block: &Block) {
        self.allocator.free_memory(block);
    }
}
