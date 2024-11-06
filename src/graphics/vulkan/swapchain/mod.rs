mod settings;

use {
    crate::{
        graphics::vulkan::{raii, Device},
        trace,
    },
    anyhow::{anyhow, Context, Result},
    ash::vk,
    std::sync::Arc,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AcquireImageStatus {
    ImageAcquired(u32),
    SwapchainNeedsRebuild,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PresentImageStatus {
    Queued,
    SwapchainNeedsRebuild,
}

/// All Vulkan resources related to the Swapchain - images, views, etc...
pub struct Swapchain {
    raw: Arc<raii::Swapchain>,
    pub extent: vk::Extent2D,
    pub format: vk::SurfaceFormatKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<raii::ImageView>,
    device: Arc<Device>,
}

impl Swapchain {
    pub fn new(
        device: Arc<Device>,
        framebuffer_size: (u32, u32),
        previous_swapchain: Option<vk::SwapchainKHR>,
    ) -> Result<Arc<Self>> {
        let (raw, extent, format) = settings::create_swapchain(
            &device,
            framebuffer_size,
            previous_swapchain,
        )
        .with_context(trace!("Unable to initialize swapchain!"))?;

        let images = unsafe { raw.ext.get_swapchain_images(raw.raw)? };
        let mut image_views = vec![];
        for image in &images {
            let create_info = vk::ImageViewCreateInfo {
                image: *image,
                view_type: vk::ImageViewType::TYPE_2D,
                format: format.format,
                components: vk::ComponentMapping::default(),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            };
            image_views.push(raii::ImageView::new(
                device.logical_device.clone(),
                &create_info,
            )?);
        }

        Ok(Arc::new(Self {
            raw,
            extent,
            format,
            images,
            image_views,
            device,
        }))
    }

    /// Returns the non-owning Vulkan swapchain handle.
    pub fn raw(&self) -> vk::SwapchainKHR {
        self.raw.raw
    }

    pub fn acquire_image(
        &self,
        image_ready_semaphore: vk::Semaphore,
    ) -> Result<AcquireImageStatus> {
        let result = unsafe {
            self.raw.ext.acquire_next_image(
                self.raw.raw,
                u64::MAX,
                image_ready_semaphore,
                vk::Fence::null(),
            )
        };
        match result {
            Ok((index, false)) => Ok(AcquireImageStatus::ImageAcquired(index)),
            Ok((_, true)) => {
                // true indicates that the swapchain is suboptimal
                Ok(AcquireImageStatus::SwapchainNeedsRebuild)
            }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                Ok(AcquireImageStatus::SwapchainNeedsRebuild)
            }
            Err(err) => Err(anyhow!(err))
                .with_context(trace!("Unable to acquire swapchain image!")),
        }
    }

    pub fn present_image(
        &self,
        wait_semaphore: vk::Semaphore,
        image_index: u32,
    ) -> Result<PresentImageStatus> {
        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: &wait_semaphore,
            swapchain_count: 1,
            p_swapchains: &self.raw.raw,
            p_image_indices: &image_index,
            ..Default::default()
        };
        let result = unsafe {
            self.raw
                .ext
                .queue_present(self.device.graphics_queue, &present_info)
        };
        match result {
            Ok(false) => Ok(PresentImageStatus::Queued),
            Ok(true) => Ok(PresentImageStatus::SwapchainNeedsRebuild),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                Ok(PresentImageStatus::SwapchainNeedsRebuild)
            }
            Err(err) => Err(err)
                .with_context(trace!("Unable to present swapchain image!")),
        }
    }
}

impl std::fmt::Debug for Swapchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Swapchain")
            .field("raw", &self.raw)
            .field("extent", &self.extent)
            .field("format", &self.format)
            .field("images", &self.images)
            .field("image_views", &self.image_views)
            .finish()
    }
}
