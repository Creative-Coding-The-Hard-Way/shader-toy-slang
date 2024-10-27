use {
    crate::trace,
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
}

impl Block {
    /// Creates a new block.
    pub(super) fn new(
        offset: u64,
        size: u64,
        memory: vk::DeviceMemory,
        mapped_ptr: *mut std::ffi::c_void,
    ) -> Self {
        Self {
            offset,
            size,
            memory,
            mapped_ptr,
        }
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
        if offset >= self.size || offset + size >= self.size {
            bail!(trace!(
                "Subregion at {} with size {:?} is out of bounds! {:#?}",
                offset,
                PrettyBytes(size),
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
}

/// Used to pretty-print the size of a block.
struct PrettyBytes(u64);

impl std::fmt::Debug for PrettyBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0;
        f.write_fmt(format_args!("{}", size))?;
        let power = (size as f64).log(1024.0).floor() as i32;
        let div = 1024.0_f64.powi(power);
        let human_size = size as f64 / div;
        match power {
            1 => {
                f.write_fmt(format_args!(" ({:.2} kb)", human_size))?;
            }
            2 => {
                f.write_fmt(format_args!(" ({:.2} mb)", human_size))?;
            }
            3 => {
                f.write_fmt(format_args!(" ({:.2} gb)", human_size))?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("offset", &self.offset)
            .field("size", &PrettyBytes(self.size))
            .field("memory", &self.memory)
            .field("mapped_ptr", &self.mapped_ptr)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use {super::*, std::ffi::c_void};

    #[test]
    pub fn subregion_should_fail_when_offset_out_of_bounds() {
        // initial offset is out of bounds
        assert!(Block {
            offset: 2,
            size: 100,
            memory: vk::DeviceMemory::null(),
            mapped_ptr: std::ptr::null_mut()
        }
        .subregion(100, 0)
        .is_err());

        // offset+size is out of bounds
        assert!(Block {
            offset: 2,
            size: 100,
            memory: vk::DeviceMemory::null(),
            mapped_ptr: std::ptr::null_mut()
        }
        .subregion(50, 50)
        .is_err());
    }

    #[test]
    pub fn subregion_should_use_cumulative_offset() -> Result<()> {
        let block = Block {
            offset: 5,
            size: 100,
            memory: vk::DeviceMemory::null(),
            mapped_ptr: std::ptr::null_mut(),
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
