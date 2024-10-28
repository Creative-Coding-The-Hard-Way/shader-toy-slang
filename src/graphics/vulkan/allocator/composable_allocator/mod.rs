mod device_allocator;
mod reporting_allocator;

use {
    self::{
        device_allocator::DeviceAllocator,
        reporting_allocator::ReportingAllocator,
    },
    crate::graphics::vulkan::{allocator::AllocationRequirements, raii, Block},
    anyhow::Result,
    std::sync::Arc,
};

pub trait ComposableAllocator {
    /// Returns true if this allocator owns the given block.
    fn owns(&self, block: &Block) -> bool;

    /// Allocate a block of memory.
    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block>;

    /// Free a block of memory.
    fn free_memory(&mut self, block: &Block);
}

pub fn create_allocator(
    logical_device: Arc<raii::Device>,
) -> impl ComposableAllocator {
    ReportingAllocator::new(
        "DeviceAllocator",
        DeviceAllocator::new(logical_device),
    )
}
