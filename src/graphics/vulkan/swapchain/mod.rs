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

/// The Vulkan swapchain and associated resources.
pub struct Swapchain {
    swapchain: Arc<raii::Swapchain>,
    extent: vk::Extent2D,
    format: vk::SurfaceFormatKHR,
    images: Vec<vk::Image>,
    image_views: Vec<raii::ImageView>,
    device: Arc<Device>,
}

impl Swapchain {
    /// Creates a new Vulkan swapchain.
    pub fn new(
        device: Arc<Device>,
        framebuffer_size: (u32, u32),
        previous_swapchain: Option<vk::SwapchainKHR>,
    ) -> Result<Arc<Self>> {
        let (swapchain, extent, format) = settings::create_swapchain(
            &device,
            framebuffer_size,
            previous_swapchain,
        )
        .with_context(trace!("Unable to initialize swapchain!"))?;

        let images =
            unsafe { swapchain.ext.get_swapchain_images(swapchain.raw)? };
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
            swapchain,
            extent,
            format,
            images,
            image_views,
            device,
        }))
    }

    /// Returns the non-owning Vulkan swapchain handle.
    pub fn raw(&self) -> vk::SwapchainKHR {
        self.swapchain.raw
    }

    /// Returns the Swapchain's current extent.
    pub fn extent(&self) -> vk::Extent2D {
        self.extent
    }

    /// Returns the Swapchain's image format.
    pub fn format(&self) -> vk::Format {
        self.format.format
    }

    /// Returns the Swapchain's image handles.
    pub fn images(&self) -> &[vk::Image] {
        &self.images
    }

    /// Returns the Swapchain's image views.
    ///
    /// Views are paired 1-1 with images of the same index.
    pub fn image_views(&self) -> &[raii::ImageView] {
        &self.image_views
    }

    /// Acquires the index for the next swapchain image.
    ///
    /// * `image_ready_semaphore` - A Vulkan semaphore to signal when the
    ///   swapchain image is ready. This can be `vk::Semaphore::null()` if not
    ///   required.
    pub fn acquire_image(
        &self,
        image_ready_semaphore: vk::Semaphore,
    ) -> Result<AcquireImageStatus> {
        let result = unsafe {
            self.swapchain.ext.acquire_next_image(
                self.swapchain.raw,
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

    /// Presents the swapchain image.
    ///
    /// * `wait_semaphore` - Image presentation waits for the semaphore to be
    ///   signalled.
    /// * `image_index` - The index of the swapchain image being presented. This
    ///   must come from a prior call to [Self::acquire_image].
    pub fn present_image(
        &self,
        wait_semaphore: vk::Semaphore,
        image_index: u32,
    ) -> Result<PresentImageStatus> {
        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: &wait_semaphore,
            swapchain_count: 1,
            p_swapchains: &self.swapchain.raw,
            p_image_indices: &image_index,
            ..Default::default()
        };
        let result = unsafe {
            self.swapchain
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
            .field("swapchain", &self.swapchain)
            .field("extent", &self.extent)
            .field("format", &self.format)
            .field("images", &self.images)
            .field("image_views", &self.image_views)
            .finish()
    }
}
