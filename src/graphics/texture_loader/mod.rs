use {
    crate::{
        graphics::vulkan::{Device, OwnedBlock, SyncCommands},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    image::ImageReader,
    std::{path::PathBuf, sync::Arc},
};

mod texture;
mod transfer_buffer;

pub use self::texture::Texture;
use self::transfer_buffer::TransferBuffer;

/// Responsible for loading textures from files into usable GPU resources.
pub struct TextureLoader {
    sync_commands: SyncCommands,
    transfer_buffer: TransferBuffer,
    device: Arc<Device>,
}

impl TextureLoader {
    /// Creates the texture loader and underlying resources.
    pub fn new(device: Arc<Device>) -> Result<Self> {
        Ok(Self {
            sync_commands: SyncCommands::new(device.clone())
                .with_context(trace!("Unable to create sync commands!"))?,
            transfer_buffer: TransferBuffer::new(device.clone(), 8)
                .with_context(trace!("Unable to create transfer buffer!"))?,
            device,
        })
    }

    pub fn load_texture(
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

        let (block, image) = OwnedBlock::allocate_image(
            self.device.allocator.clone(),
            &vk::ImageCreateInfo {
                flags: vk::ImageCreateFlags::empty(),
                image_type: vk::ImageType::TYPE_2D,
                format: vk::Format::R8G8B8A8_UNORM,
                extent: vk::Extent3D {
                    width,
                    height,
                    depth: 1,
                },
                mip_levels: 1,
                array_layers: 1,
                samples: vk::SampleCountFlags::TYPE_1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::SAMPLED,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &self
                    .device
                    .graphics_queue_family_index,
                initial_layout: vk::ImageLayout::UNDEFINED,
                ..Default::default()
            },
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )
        .with_context(trace!("Error while creating image for {:?}", path))?;

        unsafe {
            // SAFE: because the transfer buffer is not in use
            self.transfer_buffer
                .upload_bytes(image_file.as_raw())
                .with_context(trace!(
                    "Error while copying data to the transfer buffer!"
                ))?;
        }

        let device = &self.device;
        let transfer_buffer = self.transfer_buffer.buffer();
        self.sync_commands
            .submit_and_wait(|cmd| unsafe {
                device.cmd_pipeline_barrier(
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
                            .device
                            .graphics_queue_family_index,
                        dst_queue_family_index: self
                            .device
                            .graphics_queue_family_index,
                        image: image.raw,
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
                device.cmd_copy_buffer_to_image(
                    cmd,
                    transfer_buffer,
                    image.raw,
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
                device.cmd_pipeline_barrier(
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
                            .device
                            .graphics_queue_family_index,
                        dst_queue_family_index: self
                            .device
                            .graphics_queue_family_index,
                        image: image.raw,
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

        Ok(Texture::builder()
            .path(path)
            .width(width)
            .height(height)
            .image(image)
            .block(block)
            .build())
    }
}
