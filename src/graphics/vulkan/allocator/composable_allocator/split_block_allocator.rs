use {
    super::ComposableAllocator,
    crate::{
        graphics::vulkan::{allocator::AllocationRequirements, Block},
        trace,
    },
    anyhow::{Context, Result},
};

/// Represents a Block of memory that has been split in half.
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

    /// Attempts to take either the first or second free block. If no blocks are
    /// available, then returns None.
    pub fn take_free_block(&mut self) -> Option<Block> {
        if !self.first_in_use {
            self.first_in_use = true;
            return Some(self.first);
        }
        if !self.second_in_use {
            self.second_in_use = true;
            return Some(self.second);
        }
        None
    }

    /// Returns the block so it can be reused.
    pub fn free_block(&mut self, block: &Block) {
        if self.first == *block {
            self.first_in_use = false;
        } else if self.second == *block {
            self.second_in_use = false;
        } else {
            log::error!("Attempted to free an invalid block! {:#?}", block);
        }
    }

    /// Returns true when the entire block is unused.
    pub fn is_empty(&self) -> bool {
        !self.first_in_use && !self.second_in_use
    }
}

/// This allocator suballocates regions of a block by splitting it in two.
///
/// New blocks are provided by the composed allocator.
pub struct SplitBlockAllocator<A: ComposableAllocator, const BLOCK_SIZE: u64> {
    split_blocks: Vec<SplitBlock>,
    allocator: A,
}

impl<A: ComposableAllocator, const BLOCK_SIZE: u64>
    SplitBlockAllocator<A, BLOCK_SIZE>
{
    pub fn new(allocator: A) -> Self {
        Self {
            split_blocks: Vec::new(),
            allocator,
        }
    }
}

impl<A: ComposableAllocator, const BLOCK_SIZE: u64> ComposableAllocator
    for SplitBlockAllocator<A, BLOCK_SIZE>
{
    fn owns(&self, block: &Block) -> bool {
        self.split_blocks.iter().any(|split| split.contains(block))
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        if requirements.allocation_size > BLOCK_SIZE
            || BLOCK_SIZE % requirements.alignment != 0
        {
            // Give up immediately if the block size is too small
            return self.allocator.allocate_memory(requirements);
        }

        // check to see if any of the existing split blocks have space
        let sub_block = self
            .split_blocks
            .iter_mut()
            .find_map(|split_block| split_block.take_free_block());

        if let Some(block) = sub_block {
            return Ok(block);
        }

        // allocate a new block and split it instead
        let block_requirements = AllocationRequirements {
            alignment: BLOCK_SIZE,
            allocation_size: BLOCK_SIZE,
            ..requirements
        };
        let mut new_split_block = SplitBlock::new(
            self.allocator
                .allocate_memory(block_requirements)
                .with_context(trace!(
                    "Unable to allocate a new block with requirements: {:#?}",
                    block_requirements
                ))?,
        )?;

        let block = new_split_block.take_free_block().unwrap();
        self.split_blocks.push(new_split_block);

        Ok(block)
    }

    fn free_memory(&mut self, block: &Block) {
        let result = self
            .split_blocks
            .iter_mut()
            .enumerate()
            .find(|(_, split)| split.contains(block));

        let free_index = if let Some((index, split_block)) = result {
            split_block.free_block(block);
            if split_block.is_empty() {
                index
            } else {
                return;
            }
        } else {
            // none of the split blocks own the given block, so send it up
            self.allocator.free_memory(block);
            return;
        };
        self.allocator
            .free_memory(&self.split_blocks.swap_remove(free_index).total);
    }
}
