use {
    super::ComposableAllocator,
    crate::graphics::vulkan::{allocator::AllocationRequirements, Block},
    anyhow::Result,
};

pub struct ReportingAllocator<A: ComposableAllocator> {
    allocator: A,
    label: String,
}

impl<A: ComposableAllocator> ReportingAllocator<A> {
    /// Creates a new reporting allocator with the given label.
    pub fn new(label: impl Into<String>, allocator: A) -> Self {
        Self {
            allocator,
            label: label.into(),
        }
    }
}

impl<A: ComposableAllocator> ComposableAllocator for ReportingAllocator<A> {
    fn owns(&self, block: &Block) -> bool {
        let owns = self.allocator.owns(block);
        log::trace!("{} owns {:#?}?: {}", self.label, block, owns);
        owns
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        let result = self.allocator.allocate_memory(requirements);
        log::trace!(
            "{} allocate with requirements {:#?}:\n{:#?}",
            self.label,
            requirements,
            result
        );
        result
    }

    fn free_memory(&mut self, block: &Block) {
        log::trace!("{} free {:#?}", self.label, block);
        self.allocator.free_memory(block);
    }
}
