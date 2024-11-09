use {
    crate::{
        graphics::vulkan::{raii, Instance},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// Select a physical device based on the application's requried features and
/// properties.
pub fn pick_suitable_device(
    instance: &Instance,
    surface_khr: &raii::Surface,
) -> Result<vk::PhysicalDevice> {
    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .with_context(trace!("Unable to enumerate physical devices!"))?
    };

    log::trace!("Searching for suitable physical device...");

    let mut preferred_device = None;

    for physical_device in physical_devices {
        let properties =
            unsafe { instance.get_physical_device_properties(physical_device) };
        let name = properties.device_name_as_c_str().unwrap_or_default();

        log::trace!("Check device {:?}", name);
        log::trace!("Device properties: {:#?}", properties);

        let has_features = has_required_features(instance, physical_device);
        let has_queues = has_required_queues(instance, physical_device);
        let has_extensions = has_required_extensions(instance, physical_device);
        let has_surface_formats =
            has_required_surface_formats(surface_khr, physical_device)?;

        log::trace!(
            indoc::indoc! {"
                Device: {:?}
                 - has_required_features: {}
                 - has_required_queues: {}
                 - has_required_extensions: {}
                 - has_required_surface_formats: {}
            "},
            name,
            has_features,
            has_queues,
            has_extensions,
            has_surface_formats,
        );

        if has_features
            && has_queues
            && has_extensions
            && has_surface_formats
            && (preferred_device.is_none()
                || properties.device_type
                    == vk::PhysicalDeviceType::DISCRETE_GPU)
        {
            preferred_device = Some(physical_device)
        }
    }

    preferred_device.with_context(trace!("No suitable device could be found!"))
}

/// Returns true when the listed physical device has the features required by
/// the application.
fn has_required_features(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let mut features12 = vk::PhysicalDeviceVulkan12Features::default();
    let features = unsafe {
        let mut features =
            vk::PhysicalDeviceFeatures2::default().push_next(&mut features12);
        instance.get_physical_device_features2(physical_device, &mut features);
        features.features
    };

    macro_rules! check_feature12 {
        ($name:ident) => {
            if features12.$name != vk::TRUE {
                log::warn!("{} not supported!", stringify!($name));
                return false;
            }
        };
    }
    macro_rules! check_feature {
        ($name:ident) => {
            if features.$name != vk::TRUE {
                log::warn!("{} not supported!", stringify!($name));
                return false;
            }
        };
    }
    check_feature!(sampler_anisotropy);
    check_feature12!(shader_sampled_image_array_non_uniform_indexing);
    check_feature12!(descriptor_binding_partially_bound);
    check_feature12!(runtime_descriptor_array);

    true
}

fn has_required_queues(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let queue_propertes = unsafe {
        instance.get_physical_device_queue_family_properties(physical_device)
    };
    log::trace!("{:#?}", queue_propertes);

    queue_propertes.iter().any(|properties| {
        properties.queue_flags.contains(vk::QueueFlags::GRAPHICS)
    })
}

fn has_required_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let extension_properties = unsafe {
        instance
            .enumerate_device_extension_properties(physical_device)
            .unwrap_or_default()
    };
    log::trace!("{:#?}", extension_properties);

    extension_properties.iter().any(|props| {
        props.extension_name_as_c_str().unwrap_or_default()
            == ash::khr::swapchain::NAME
    })
}

fn has_required_surface_formats(
    surface_khr: &raii::Surface,
    physical_device: vk::PhysicalDevice,
) -> Result<bool> {
    let formats = unsafe {
        surface_khr.ext.get_physical_device_surface_formats(
            physical_device,
            surface_khr.raw,
        )?
    };
    log::trace!("{:#?}", formats);

    let present_modes = unsafe {
        surface_khr.ext.get_physical_device_surface_present_modes(
            physical_device,
            surface_khr.raw,
        )?
    };
    log::trace!("{:#?}", present_modes);

    Ok(!formats.is_empty() && !present_modes.is_empty())
}
