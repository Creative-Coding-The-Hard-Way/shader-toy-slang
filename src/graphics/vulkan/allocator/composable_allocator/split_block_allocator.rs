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

/// Represents a Block of memory that has been split into two equal chunks.
struct SplitBlock {
    first_in_use: bool,
    second_in_use: bool,
    first: Block,
    second: Block,
    total: Block,
}

impl SplitBlock {
    pub fn new(block: Block) -> Result<Self> {
        let midpoint = block.size() / 2;
        Ok(Self {
            first_in_use: false,
            second_in_use: false,
            first: block.subregion(0, midpoint)?,
            second: block.subregion(midpoint, block.size() - midpoint)?,
            total: block,
        })
    }

    pub fn contains(&self, block: &Block) -> bool {
        block.is_subregion_of(&self.total)
    }
}

/// This allocator suballocates regions of a block by splitting it in two.
///
/// New blocks are provided by the composed allocator.
pub struct SplitBlockAllocator<A: ComposableAllocator> {
    split_blocks: Vec<SplitBlock>,
    allocator: A,
}

impl<A: ComposableAllocator> SplitBlockAllocator<A> {
    pub fn new(allocator: A) -> Self {
        Self {
            split_blocks: Vec::new(),
            allocator,
        }
    }
}

impl<A: ComposableAllocator> ComposableAllocator for SplitBlockAllocator<A> {
    fn owns(&self, block: &Block) -> bool {
        self.split_blocks.iter().any(|split| split.contains(block))
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        let block = self.allocator.allocate_memory(requirements)?;
        self.split_blocks.push(SplitBlock::new(block)?);
        Ok(block)
    }

    fn free_memory(&mut self, block: &Block) {
        let result = self
            .split_blocks
            .iter()
            .enumerate()
            .find(|(_, split)| split.contains(block));

        if let Some((index, _)) = result {
            self.split_blocks.swap_remove(index);
            self.allocator.free_memory(block);
        }
    }
}
