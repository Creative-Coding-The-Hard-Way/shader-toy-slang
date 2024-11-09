use {
    self::transfer_buffer::TransferBuffer,
    crate::{
        graphics::{
            vulkan::{SyncCommands, VulkanContext},
            Texture,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    image::ImageReader,
    std::{path::PathBuf, sync::Arc},
};

mod transfer_buffer;

pub struct TextureLoader {
    sync_commands: SyncCommands,
    transfer_buffer: TransferBuffer,
    cxt: Arc<VulkanContext>,
}

impl TextureLoader {
    /// Creates the texture loader and underlying resources.
    pub fn new(cxt: Arc<VulkanContext>) -> Result<Self> {
        Ok(Self {
            sync_commands: SyncCommands::new(cxt.clone())
                .with_context(trace!("Unable to create sync commands!"))?,
            transfer_buffer: TransferBuffer::new(cxt.clone())
                .with_context(trace!("Unable to create transfer buffer!"))?,
            cxt,
        })
    }

    pub fn load_from_file(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> Result<Texture> {
        let path: PathBuf = path.into();

        let image_file = ImageReader::open(&path)
            .with_context(trace!("Unable to open image at {:?}", path))?
            .decode()
            .with_context(trace!("Unable to decode image at {:?}", path))?
            .to_rgba8();
        let (width, height) = image_file.dimensions();

        let texture = Texture::builder()
            .ctx(&self.cxt)
            .dimensions(image_file.dimensions())
            .format(vk::Format::R8G8B8A8_SRGB)
            .usage(
                vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::SAMPLED,
            )
            .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .build()?;

        unsafe {
            // SAFE: because the transfer buffer is not in use
            self.transfer_buffer
                .upload_bytes(image_file.as_raw())
                .with_context(trace!(
                    "Error while copying data to the transfer buffer!"
                ))?;
        }

        let cxt = &self.cxt;
        let transfer_buffer = self.transfer_buffer.buffer();
        self.sync_commands
            .submit_and_wait(|cmd| unsafe {
                cxt.cmd_pipeline_barrier(
                    cmd,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[], // memory barriers
                    &[], // buffer barriers
                    &[vk::ImageMemoryBarrier {
                        src_access_mask: vk::AccessFlags::empty(),
                        dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                        old_layout: vk::ImageLayout::UNDEFINED,
                        new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        src_queue_family_index: self
                            .cxt
                            .graphics_queue_family_index,
                        dst_queue_family_index: self
                            .cxt
                            .graphics_queue_family_index,
                        image: texture.image(),
                        subresource_range: vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        },
                        ..Default::default()
                    }],
                );
                cxt.cmd_copy_buffer_to_image(
                    cmd,
                    transfer_buffer,
                    texture.image(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[vk::BufferImageCopy {
                        buffer_offset: 0,
                        buffer_row_length: 0,
                        buffer_image_height: 0,
                        image_subresource: vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            mip_level: 0,
                            base_array_layer: 0,
                            layer_count: 1,
                        },
                        image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                        image_extent: vk::Extent3D {
                            width,
                            height,
                            depth: 1,
                        },
                    }],
                );
                cxt.cmd_pipeline_barrier(
                    cmd,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                    vk::DependencyFlags::empty(),
                    &[], // memory barriers
                    &[], // buffer barriers
                    &[vk::ImageMemoryBarrier {
                        src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                        dst_access_mask: vk::AccessFlags::empty(),
                        old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                        new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                        src_queue_family_index: self
                            .cxt
                            .graphics_queue_family_index,
                        dst_queue_family_index: self
                            .cxt
                            .graphics_queue_family_index,
                        image: texture.image(),
                        subresource_range: vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        },
                        ..Default::default()
                    }],
                );

                Ok(())
            })
            .with_context(trace!(
                "Error while copying data to image memory!"
            ))?;

        Ok(texture)
    }
}
