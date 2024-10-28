use {
    super::ComposableAllocator,
    crate::graphics::vulkan::{allocator::AllocationRequirements, Block},
    anyhow::Result,
    std::collections::{hash_map::Entry, HashMap},
};

/// Organizes allocation requests by the memory type index.
///
/// Allocations from the same memory type index can be broken apart and
/// suballocated, etc...
pub struct TypeIndexAllocator {
    allocators: HashMap<u32, Box<dyn ComposableAllocator>>,
    type_index_factory: Box<dyn Fn(u32) -> Box<dyn ComposableAllocator>>,
}

impl TypeIndexAllocator {
    pub fn new<T, F>(type_index_factory: F) -> Self
    where
        T: ComposableAllocator + 'static,
        F: Fn(u32) -> T + 'static,
    {
        let type_index_factory = move |index| -> Box<dyn ComposableAllocator> {
            let alloc = type_index_factory(index);
            Box::new(alloc)
        };
        Self {
            allocators: HashMap::with_capacity(4),
            type_index_factory: Box::new(type_index_factory),
        }
    }
}

impl ComposableAllocator for TypeIndexAllocator {
    fn owns(&self, block: &Block) -> bool {
        self.allocators
            .values()
            .any(|allocator| allocator.owns(block))
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        if let Entry::Vacant(e) =
            self.allocators.entry(requirements.memory_type_index)
        {
            e.insert((self.type_index_factory)(requirements.memory_type_index));
        }

        self.allocators
            .get_mut(&requirements.memory_type_index)
            .unwrap()
            .allocate_memory(requirements)
    }

    fn free_memory(&mut self, block: &Block) {
        self.allocators
            .get_mut(&block.memory_type_index())
            .expect("Invalid block memory type index!")
            .free_memory(block);
    }
}
