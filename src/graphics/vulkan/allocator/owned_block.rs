use {
    crate::{
        graphics::vulkan::{raii, Allocator, Block},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// A block allocation that frees itself when dropped.
#[derive(Debug)]
pub struct OwnedBlock {
    block: Block,
    allocator: Arc<Allocator>,
}

impl OwnedBlock {
    /// Creates an image and allocates memory to back it.
    ///
    /// The image is bound to the memory prior to return, so the caller can use
    /// it right away.
    pub fn allocate_image(
        allocator: Arc<Allocator>,
        image_create_info: &vk::ImageCreateInfo,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<(raii::Image, Self)> {
        let image = raii::Image::new(
            allocator.logical_device.clone(),
            image_create_info,
        )
        .with_context(trace!("Unable to create image!"))?;

        let requirements = {
            let mut dedicated = vk::MemoryDedicatedRequirements::default();
            let requirements = unsafe {
                let mut out = vk::MemoryRequirements2::default()
                    .push_next(&mut dedicated);

                allocator.logical_device.get_image_memory_requirements2(
                    &vk::ImageMemoryRequirementsInfo2 {
                        image: image.raw,
                        ..Default::default()
                    },
                    &mut out,
                );

                out.memory_requirements
            };
            requirements
        };

        let block = allocator
            .allocate_memory(&requirements, flags)
            .with_context(trace!("Unable to allocate memory for image!"))?;

        unsafe {
            allocator
                .logical_device
                .bind_image_memory(image.raw, block.memory(), block.offset())
                .with_context(trace!("Unable to bind image memory!"))?;
        };

        Ok((image, Self { block, allocator }))
    }

    /// Creates a buffer and allocates memory to back it.
    ///
    /// The buffer is bound to the memory prior to return, so the caller can use
    /// it right away.
    pub fn allocate_buffer(
        allocator: Arc<Allocator>,
        buffer_create_info: &vk::BufferCreateInfo,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<(raii::Buffer, OwnedBlock)> {
        let buffer = raii::Buffer::new(
            allocator.logical_device.clone(),
            buffer_create_info,
        )
        .with_context(trace!("Unable to create buffer!"))?;

        let requirements = {
            let mut dedicated = vk::MemoryDedicatedRequirements::default();
            let requirements = unsafe {
                let mut out = vk::MemoryRequirements2::default()
                    .push_next(&mut dedicated);

                allocator.logical_device.get_buffer_memory_requirements2(
                    &vk::BufferMemoryRequirementsInfo2 {
                        buffer: buffer.raw,
                        ..Default::default()
                    },
                    &mut out,
                );

                out.memory_requirements
            };
            requirements
        };

        let block = allocator
            .allocate_memory(&requirements, flags)
            .with_context(trace!("Unable to allocate memory for buffer!"))?;

        unsafe {
            allocator
                .logical_device
                .bind_buffer_memory(buffer.raw, block.memory(), block.offset())
                .with_context(trace!("Unable to bind buffer to memory!"))?;
        };

        Ok((buffer, Self { block, allocator }))
    }
}

impl std::ops::Deref for OwnedBlock {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl Drop for OwnedBlock {
    fn drop(&mut self) {
        self.allocator.free(&self.block);
    }
}
