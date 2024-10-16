use {
    crate::{
        graphics::vulkan::{raii, Device},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

pub fn create_swapchain(
    device: Arc<Device>,
    framebuffer_size: (u32, u32),
    previous_swapchain: Option<vk::SwapchainKHR>,
) -> Result<(Arc<raii::Swapchain>, vk::Extent2D, vk::SurfaceFormatKHR)> {
    let capabilities = unsafe {
        device
            .surface_khr
            .ext
            .get_physical_device_surface_capabilities(
                device.physical_device,
                device.surface_khr.raw,
            )
            .with_context(trace!("Unable to get surface capabilities!"))?
    };
    log::trace!("Device capabilities:\n{:#?}", capabilities);

    let format = select_image_format(&device)?;
    let extent = select_image_extent(&capabilities, framebuffer_size);
    let queue_families = [device.graphics_queue_family_index];
    let create_info = vk::SwapchainCreateInfoKHR {
        surface: device.surface_khr.raw,
        min_image_count: select_image_count(&capabilities),
        image_format: format.format,
        image_color_space: format.color_space,
        image_extent: extent,
        image_array_layers: 1,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        image_sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 1,
        p_queue_family_indices: queue_families.as_ptr(),
        pre_transform: capabilities.current_transform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode: select_present_mode(&device)?,
        clipped: vk::TRUE,
        old_swapchain: previous_swapchain.unwrap_or(vk::SwapchainKHR::null()),
        ..Default::default()
    };
    let swapchain =
        raii::Swapchain::new(device.logical_device.clone(), &create_info)?;
    log::trace!(
        indoc::indoc!(
            "
            Created swapchain:
              - swapchain: {:#?}
              - extent: {:?}
              - format: {:?}
            "
        ),
        swapchain,
        extent,
        format
    );
    Ok((swapchain, extent, format))
}

/// Pick the desired image format for the swapchain.
fn select_image_format(device: &Device) -> Result<vk::SurfaceFormatKHR> {
    let surface_formats = unsafe {
        device
            .surface_khr
            .ext
            .get_physical_device_surface_formats(
                device.physical_device,
                device.surface_khr.raw,
            )
            .with_context(trace!(
                "Error while listing available surface formats!"
            ))?
    };
    log::trace!("Formats supported by device\n{:#?}", surface_formats);

    let preferred = surface_formats.iter().find(|surface_format| {
        let has_color_space =
            surface_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR;
        let has_format = surface_format.format == vk::Format::B8G8R8A8_SRGB;
        has_color_space && has_format
    });

    let format = preferred.or(surface_formats.first()).with_context(trace!(
        "Unable to find a suitable surface format for the swapchain!"
    ))?;

    Ok(*format)
}

fn select_image_count(capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let count = capabilities.min_image_count + 2;
    if capabilities.max_image_count > 0 {
        count.clamp(capabilities.min_image_count, capabilities.max_image_count)
    } else {
        count
    }
}

fn select_image_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    framebuffer_size: (u32, u32),
) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        // If current_extent does not equal u32::MAX, then it contains the
        // extent of the targeted surface and can be used directly. See:
        //
        // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html

        return capabilities.current_extent;
    }

    let (desired_width, desired_height) = framebuffer_size;
    vk::Extent2D {
        width: desired_width.clamp(
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width,
        ),
        height: desired_height.clamp(
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height,
        ),
    }
}

fn select_present_mode(device: &Device) -> Result<vk::PresentModeKHR> {
    let present_modes = unsafe {
        device
            .surface_khr
            .ext
            .get_physical_device_surface_present_modes(
                device.physical_device,
                device.surface_khr.raw,
            )?
    };
    log::trace!("Present modes for device:\n{:#?}", present_modes);
    if present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
        Ok(vk::PresentModeKHR::MAILBOX)
    } else {
        Ok(vk::PresentModeKHR::FIFO)
    }
}
