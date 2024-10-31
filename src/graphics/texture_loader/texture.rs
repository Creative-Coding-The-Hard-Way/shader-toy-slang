use {
    crate::graphics::vulkan::{raii, OwnedBlock},
    ash::vk,
    bon::Builder,
    std::path::PathBuf,
};

/// A Texture is a Vulkan Image paired with backing GPU memory and metadata.
///
/// # Safety
///
/// Textures automatically destroy any owned resources when they're dropped.
/// The application is responsible for synchronizing GPU access to the texture
/// before allowing it to drop.
#[derive(Debug, Builder)]
pub struct Texture {
    path: PathBuf,
    width: u32,
    height: u32,
    image: raii::Image,
    block: OwnedBlock,
}

impl Texture {
    /// A non-owning handle for the underlying Vulkan image resource.
    pub fn image(&self) -> vk::Image {
        self.image.raw
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
