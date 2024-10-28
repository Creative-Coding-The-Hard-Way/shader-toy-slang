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

/// Tries to handle all allocations with A. Falls back to B on failure.
pub struct FallbackAllocator<A: ComposableAllocator, B: ComposableAllocator> {
    primary: A,
    fallback: B,
}

impl<A: ComposableAllocator, B: ComposableAllocator> FallbackAllocator<A, B> {
    pub fn new(primary: A, fallback: B) -> Self {
        Self { primary, fallback }
    }
}

impl<A: ComposableAllocator, B: ComposableAllocator> ComposableAllocator
    for FallbackAllocator<A, B>
{
    fn owns(&self, block: &Block) -> bool {
        self.primary.owns(block) || self.fallback.owns(block)
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        self.primary
            .allocate_memory(requirements)
            .or_else(|_| self.fallback.allocate_memory(requirements))
    }

    fn free_memory(&mut self, block: &Block) {
        if self.primary.owns(block) {
            self.primary.free_memory(block);
        } else {
            self.fallback.free_memory(block);
        }
    }
}
