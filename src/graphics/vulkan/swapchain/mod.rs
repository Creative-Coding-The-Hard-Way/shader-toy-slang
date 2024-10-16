mod settings;

use {
    crate::{
        graphics::vulkan::{raii, Device},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// All Vulkan resources related to the Swapchain - images, views, etc...
pub struct Swapchain {
    pub raw: Arc<raii::Swapchain>,
    pub extent: vk::Extent2D,
    pub format: vk::SurfaceFormatKHR,
}

impl Swapchain {
    pub fn new(
        device: Arc<Device>,
        framebuffer_size: (u32, u32),
    ) -> Result<Arc<Self>> {
        let (raw, extent, format) =
            settings::create_swapchain(device, framebuffer_size, None)
                .with_context(trace!("Unable to initialize swapchain!"))?;
        Ok(Arc::new(Self {
            raw,
            extent,
            format,
        }))
    }
}
