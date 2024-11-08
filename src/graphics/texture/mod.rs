pub mod texture_loader;

use {
    crate::{
        graphics::vulkan::{raii, OwnedBlock, VulkanContext},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    bon::bon,
};

/// A Texture is a Vulkan Image paired with backing GPU memory and metadata.
///
/// # Safety
///
/// Textures automatically destroy any owned resources when they're dropped.
/// The application is responsible for synchronizing GPU access to the texture
/// before allowing it to drop.
#[derive(Debug)]
pub struct Texture {
    _name: String,
    width: u32,
    height: u32,
    image_view: raii::ImageView,
    image: raii::Image,
    block: OwnedBlock,
}

#[bon]
impl Texture {
    /// Creates a new texture with the given dimensions.
    ///
    /// Texture memory contains undefined data until fresh data is uploaded by
    /// some means.
    #[builder]
    pub fn new(
        ctx: &VulkanContext,
        name: String,
        dimensions: (u32, u32),
        format: vk::Format,
        usage: vk::ImageUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        let (width, height) = dimensions;
        let (block, image) = OwnedBlock::allocate_image(
            ctx.allocator.clone(),
            &vk::ImageCreateInfo {
                flags: vk::ImageCreateFlags::empty(),
                image_type: vk::ImageType::TYPE_2D,
                format,
                extent: vk::Extent3D {
                    width,
                    height,
                    depth: 1,
                },
                mip_levels: 1,
                array_layers: 1,
                samples: vk::SampleCountFlags::TYPE_1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &ctx.graphics_queue_family_index,
                initial_layout: vk::ImageLayout::UNDEFINED,
                ..Default::default()
            },
            memory_property_flags,
        )
        .with_context(trace!("Error while creating image for {:?}", name))?;

        let image_view = raii::ImageView::new(
            ctx.device.clone(),
            &vk::ImageViewCreateInfo {
                flags: vk::ImageViewCreateFlags::empty(),
                image: image.raw,
                view_type: vk::ImageViewType::TYPE_2D,
                format,
                components: vk::ComponentMapping::default(),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            },
        )
        .with_context(trace!("Unable to create image view!"))?;

        Ok(Self {
            _name: name,
            width,
            height,
            image_view,
            image,
            block,
        })
    }

    /// A non-owning handle for the underlying Vulkan image resource.
    pub fn image(&self) -> vk::Image {
        self.image.raw
    }

    /// A non-owning handle for the underlying Vulkan image view.
    pub fn view(&self) -> vk::ImageView {
        self.image_view.raw
    }

    /// A non-owning handle for the underlying GPU memory.
    pub fn memory(&self) -> vk::DeviceMemory {
        self.block.memory()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn extent(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.width(),
            height: self.height(),
        }
    }
}
