mod dedicated_allocator;
mod device_allocator;
mod fallback_allocator;
mod reporting_allocator;
mod round_up_allocator;
mod split_block_allocator;
mod type_index_allocator;

use {
    self::{
        device_allocator::DeviceAllocator,
        fallback_allocator::FallbackAllocator,
        reporting_allocator::LabelledAllocatorBuilder,
        split_block_allocator::SplitBlockAllocator,
        type_index_allocator::TypeIndexAllocator,
    },
    crate::graphics::vulkan::{allocator::AllocationRequirements, raii, Block},
    anyhow::Result,
    ash::vk,
    dedicated_allocator::DedicatedAllocator,
    std::{cell::RefCell, rc::Rc, sync::Arc},
};

pub fn create_system_allocator(
    logical_device: Arc<raii::Device>,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
) -> impl ComposableAllocator {
    let device_allocator = DeviceAllocator::new(logical_device.clone())
        .description("DeviceAllocator", "The raw Vulkan device allocator.")
        .shared();

    FallbackAllocator::new(
        DedicatedAllocator::new(device_allocator.clone()),
        TypeIndexAllocator::new(move |index| {
            // This is a silly implementation. By chaining split block
            // allocators, a given allocation will continue to escalate until
            // eventually asking the device for a big block of memory. From
            // there, the block will be split into two parts over and over until
            // the smallest correct block is found.
            //
            // This could certainly be made both *faster* (in CPU time) and
            // more efficient in terms of allocated memory vs used memory. Not
            // to mention fragmentation, etc...
            //
            // But this application doesn't care about that yet, so simply
            // taking a big chunk and spliting it in half over and over is
            // easy to understand and good enough for now.
            #[rustfmt::skip]
            let allocator =
                SplitBlockAllocator::<_, 1024               >::new(
                SplitBlockAllocator::<_, {1024 * 2         }>::new(
                SplitBlockAllocator::<_, {1024 * 4         }>::new(
                SplitBlockAllocator::<_, {1024 * 8         }>::new(
                SplitBlockAllocator::<_, {1024 * 16        }>::new(
                SplitBlockAllocator::<_, {1024 * 32        }>::new(
                SplitBlockAllocator::<_, {1024 * 64        }>::new(
                SplitBlockAllocator::<_, {1024 * 128       }>::new(
                SplitBlockAllocator::<_, {1024 * 256       }>::new(
                SplitBlockAllocator::<_, {1024 * 512       }>::new(
                SplitBlockAllocator::<_, {1024 * 1024      }>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 2  }>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 4  }>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 8  }>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 16 }>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 32 }>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 64 }>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 128}>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 256}>::new(
                SplitBlockAllocator::<_, {1024 * 1024 * 512}>::new(
                    device_allocator.clone(),
                ))))))))))))))))))));

            allocator.description(
                format!("IndexedAllocator:{}", index,),
                format!(
                    "This allocator is responsible for allocating blocks \
                    with memory properties, {:?}, and subdividing them for \
                    use by the application.",
                    memory_properties.memory_types[index as usize]
                        .property_flags
                ),
            )
        }),
    )
    .description(
        "ApplicationAllocator",
        "The top level allocator exposed to the application.",
    )
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
