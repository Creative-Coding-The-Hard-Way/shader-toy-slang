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
}

/// This allocator suballocates regions of a block by splitting it in two.
///
/// New blocks are provided by the composed allocator.
pub struct SplitBlockAllocator<A: ComposableAllocator> {
    blocks: Vec<SplitBlock>,
    allocator: A,
}

impl<A: ComposableAllocator> SplitBlockAllocator<A> {
    pub fn new(allocator: A) -> Self {
        Self {
            blocks: Vec::new(),
            allocator,
        }
    }
}

impl<A: ComposableAllocator> ComposableAllocator for SplitBlockAllocator<A> {
    fn owns(&self, block: &Block) -> bool {
        self.blocks.iter().any(|split| split.total == *block)
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        let block = self.allocator.allocate_memory(requirements)?;
        self.blocks.push(SplitBlock::new(block)?);
        Ok(block)
    }

    fn free_memory(&mut self, block: &Block) {
        let result = self
            .blocks
            .iter()
            .enumerate()
            .find(|(_, owned_block)| owned_block.total.eq(block));

        if let Some((index, _)) = result {
            self.blocks.swap_remove(index);
            self.allocator.free_memory(block);
        }
    }
}
