use {
    crate::{graphics::vulkan::allocator::HumanizedSize, trace},
    anyhow::{bail, Result},
    ash::vk,
};

/// Allocators return blocks of memory that can be used for Vulkan operations.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Block {
    offset: u64,
    size: u64,
    memory: vk::DeviceMemory,
    mapped_ptr: *mut std::ffi::c_void,
    memory_type_index: u32,
}

/// Blocks are not Send by default because they include the mapped_ptr.
///
/// It is safe to send the mapped ptr between threads, but the application is
/// responsible for synchronizing access to the underlying buffer via the
/// pointer.
unsafe impl Send for Block {}

/// It is generally safe to share blocks between threads. The application is
/// responsible for synchronizing access to device memory.
unsafe impl Sync for Block {}

impl Block {
    /// Creates a new block.
    pub(super) fn new(
        offset: u64,
        size: u64,
        memory: vk::DeviceMemory,
        mapped_ptr: *mut std::ffi::c_void,
        memory_type_index: u32,
    ) -> Self {
        Self {
            offset,
            size,
            memory,
            mapped_ptr,
            memory_type_index,
        }
    }

    /// Returns true when self is entirely contained by other, false otherwise.
    ///
    /// Note: partial overlaps are not considered 'subregions'.
    /// A.is_subregion_of(B) will return true iff A is fully contained by B.
    pub fn is_subregion_of(&self, other: &Block) -> bool {
        if self.memory() != other.memory() {
            // The blocks refer to different underlying DeviceMemory allocations
            return false;
        }

        let start = self.offset();
        let end = (self.offset() + self.size()) - 1;

        let starts_within =
            start >= other.offset() && start < other.offset() + other.size();
        let ends_within =
            end >= other.offset() && end < other.offset() + other.size();

        starts_within && ends_within
    }

    /// Returns a subregion of the current Block.
    ///
    /// Fails if the requested subregion is out of bounds or cannot fit within
    /// the current block.
    ///
    /// ## Params
    ///
    /// - offset: the offset in bytes from the beginning of the current block
    /// - size: the size of the subregion to return
    pub fn subregion(&self, offset: u64, size: u64) -> Result<Self> {
        if offset >= self.size || offset + size > self.size {
            bail!(trace!(
                "Subregion at {} with size {:?} is out of bounds! {:#?}",
                offset,
                HumanizedSize(size),
                self,
            )());
        }

        let mapped_ptr: *mut std::ffi::c_void = if self.mapped_ptr.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { self.mapped_ptr.byte_offset(offset as isize) }
        };

        Ok(Block {
            offset: self.offset + offset,
            size,
            memory: self.memory,
            mapped_ptr,
            memory_type_index: self.memory_type_index,
        })
    }

    /// Returns the start of the block as a byte offset into device memory.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Returns the size of the block in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Gets a non-owning copy of the underlying DeviceMemory handle.
    ///
    /// Multiple blocks can reference the underlying memory. Blocks returned by
    /// the allocator are guaranteed to not overlap.
    pub fn memory(&self) -> vk::DeviceMemory {
        self.memory
    }

    /// Returns the memory-mapped pointer for the DeviceMemory.
    ///
    /// The pointer is null() if the memory is not mapped.
    ///
    /// The pointer always points to the start of the Block and does not need to
    /// be manually adjusted to account for the offset.
    pub fn mapped_ptr(&self) -> *mut std::ffi::c_void {
        self.mapped_ptr
    }

    /// Returns the memory type index for the Block's device memory.
    pub fn memory_type_index(&self) -> u32 {
        self.memory_type_index
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("offset", &self.offset)
            .field("size", &HumanizedSize(self.size))
            .field("memory_type_index", &self.memory_type_index)
            .field("memory", &self.memory)
            .field("mapped_ptr", &self.mapped_ptr)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use {super::*, std::ffi::c_void, vk::Handle};

    #[test]
    pub fn is_subregion_of_should_be_true_for_contained_blocks() -> Result<()> {
        let block = Block::new(
            0,
            100,
            vk::DeviceMemory::null(),
            std::ptr::null_mut(),
            0,
        );
        assert!(block.subregion(99, 1)?.is_subregion_of(&block));
        assert!(block.subregion(0, 100)?.is_subregion_of(&block));
        assert!(block.subregion(50, 50)?.is_subregion_of(&block));
        assert!(block.subregion(2, 80)?.is_subregion_of(&block));
        Ok(())
    }

    #[test]
    pub fn is_subregion_of_should_be_false_for_partial_overlaps() -> Result<()>
    {
        let blk = Block::new(
            50,
            50,
            vk::DeviceMemory::null(),
            std::ptr::null_mut(),
            0,
        );
        let end_overlap = Block { offset: 75, ..blk };
        assert!(!end_overlap.is_subregion_of(&blk));

        let start_overlap = Block { offset: 25, ..blk };
        assert!(!start_overlap.is_subregion_of(&blk));
        Ok(())
    }

    #[test]
    pub fn is_subregion_of_should_be_true_for_identical_blocks() {
        let block = Block::new(
            0,
            100,
            vk::DeviceMemory::null(),
            std::ptr::null_mut(),
            0,
        );
        assert!(block.is_subregion_of(&block));
    }

    #[test]
    pub fn is_subregion_of_should_be_false_when_device_memory_does_not_match() {
        let a = Block::new(
            0,
            100,
            vk::DeviceMemory::from_raw(1), // INVALID - do not access
            std::ptr::null_mut(),
            0,
        );
        let b = Block::new(
            0,
            100,
            vk::DeviceMemory::from_raw(2), // INVALID - do not access
            std::ptr::null_mut(),
            0,
        );
        assert!(!a.is_subregion_of(&b));
    }

    #[test]
    pub fn subregion_should_fail_when_offset_out_of_bounds() {
        // initial offset is out of bounds
        assert!(Block {
            offset: 2,
            size: 100,
            memory: vk::DeviceMemory::null(),
            mapped_ptr: std::ptr::null_mut(),
            memory_type_index: 0
        }
        .subregion(100, 0)
        .is_err());

        // offset+size is out of bounds
        assert!(Block {
            offset: 2,
            size: 100,
            memory: vk::DeviceMemory::null(),
            mapped_ptr: std::ptr::null_mut(),
            memory_type_index: 0
        }
        .subregion(50, 51)
        .is_err());
    }

    #[test]
    pub fn subregion_should_use_cumulative_offset() -> Result<()> {
        let block = Block {
            offset: 5,
            size: 100,
            memory: vk::DeviceMemory::null(),
            mapped_ptr: std::ptr::null_mut(),
            memory_type_index: 0,
        };
        let sub = block.subregion(3, 80)?;
        assert!(sub.offset == 8);
        assert!(sub.size == 80);
        assert!(sub.mapped_ptr.is_null());
        Ok(())
    }

    #[test]
    pub fn subregion_should_update_mapped_ptr() -> Result<()> {
        let buffer = [0_u8; 100];
        let block = Block {
            offset: 0,
            size: 100,
            memory: vk::DeviceMemory::null(),
            mapped_ptr: buffer.as_ptr() as *mut c_void,
            memory_type_index: 0,
        };
        let sub = block.subregion(3, 80)?;
        let ptr_offset =
            unsafe { sub.mapped_ptr.byte_offset_from(block.mapped_ptr) };
        assert!(ptr_offset == (sub.offset as isize));

        let sub2 = sub.subregion(15, 30)?;
        assert!(
            unsafe { sub2.mapped_ptr.byte_offset_from(block.mapped_ptr) }
                == 15 + 3
        );
        assert!(
            unsafe { sub2.mapped_ptr.byte_offset_from(sub.mapped_ptr) } == 15
        );
        Ok(())
    }
}
