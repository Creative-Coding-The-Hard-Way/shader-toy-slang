mod dedicated_allocator;
mod device_allocator;
mod fallback_allocator;
mod reporting_allocator;
mod round_up_allocator;

use {
    self::{
        device_allocator::DeviceAllocator,
        fallback_allocator::FallbackAllocator,
        reporting_allocator::LabelledAllocatorBuilder,
        round_up_allocator::RoundUpAllocator,
    },
    crate::graphics::vulkan::{allocator::AllocationRequirements, raii, Block},
    anyhow::Result,
    dedicated_allocator::DedicatedAllocator,
    std::{cell::RefCell, rc::Rc, sync::Arc},
};

pub fn create_system_allocator(
    logical_device: Arc<raii::Device>,
) -> impl ComposableAllocator {
    let device_allocator = DeviceAllocator::new(logical_device.clone())
        .label("DeviceAllocator")
        .shared();

    RoundUpAllocator::new(
        1024 * 1024,
        FallbackAllocator::new(
            DedicatedAllocator::new(device_allocator.clone())
                .label("Dedicated"),
            device_allocator,
        ),
    )
    .label("SystemAllocator")
}

/// Composable Allocators are externally synchronized and can have mutable
/// state.
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

    /// Replace self with a type-erased instance that can be cloned.
    fn shared(self) -> SharedAllocator
    where
        Self: Sized + 'static,
    {
        SharedAllocator::new(self)
    }
}

/// A reference counted shared instance of an allocator.
#[derive(Clone)]
pub struct SharedAllocator {
    allocator: Rc<RefCell<dyn ComposableAllocator>>,
}

impl SharedAllocator {
    pub fn new<A: ComposableAllocator + 'static>(allocator: A) -> Self {
        Self {
            allocator: Rc::new(RefCell::new(allocator)),
        }
    }
}

impl ComposableAllocator for SharedAllocator {
    fn owns(&self, block: &Block) -> bool {
        self.allocator.borrow().owns(block)
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        self.allocator.borrow_mut().allocate_memory(requirements)
    }

    fn free_memory(&mut self, block: &Block) {
        self.allocator.borrow_mut().free_memory(block);
    }
}