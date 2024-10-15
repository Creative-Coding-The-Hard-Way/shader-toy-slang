mod instance;
mod physical_device;

use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk::{self, QueueFlags},
    std::sync::Arc,
};

pub use self::instance::Instance;

/// The Vulkan device is the logical handle for all Vulkan resource operations.
pub struct Device {
    pub instance: Instance,
    pub surface_khr: Arc<raii::Surface>,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Arc<raii::Device>,
}

impl Device {
    pub fn new(window: &glfw::Window) -> Result<Self> {
        let instance = Instance::for_window("Shader-Toy-Slang", window)
            .with_context(trace!("Unable to create vulkan instance!"))?;

        let surface_khr =
            raii::Surface::from_glfw_window(instance.ash.clone(), window)
                .with_context(trace!(
                    "Unable to create Vulkan surface from glfw window!"
                ))?;

        let physical_device =
            physical_device::pick_suitable_device(&instance, &surface_khr)
                .with_context(trace!(
                    "Error while picking a suitable physical device!"
                ))?;

        let queue_family_properties = unsafe {
            instance
                .ash
                .get_physical_device_queue_family_properties(physical_device)
        };

        let (graphics_present_queue, _) = queue_family_properties
            .iter()
            .enumerate()
            .find(|(index, properties)| {
                let supports_present = unsafe {
                    surface_khr
                        .ext
                        .get_physical_device_surface_support(
                            physical_device,
                            *index as u32,
                            surface_khr.raw,
                        )
                        .unwrap_or(false)
                };
                supports_present
                    && properties.queue_flags.contains(QueueFlags::GRAPHICS)
            })
            .with_context(trace!(
                "Unable to find a queue that supports GRAPHICS."
            ))?;
        let queue_priorities = [1.0f32];
        let queue_create_infos = [vk::DeviceQueueCreateInfo {
            queue_family_index: graphics_present_queue as u32,
            queue_count: 1,
            p_queue_priorities: queue_priorities.as_ptr(),
            ..Default::default()
        }];
        let extensions = [ash::khr::swapchain::NAME.as_ptr()];
        let features = vk::PhysicalDeviceFeatures {
            geometry_shader: vk::TRUE,
            ..Default::default()
        };
        let create_info = vk::DeviceCreateInfo {
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            p_enabled_features: &features,
            ..Default::default()
        };
        let logical_device = raii::Device::new(
            instance.ash.clone(),
            physical_device,
            &create_info,
        )?;

        Ok(Self {
            instance,
            surface_khr,
            physical_device,
            logical_device,
        })
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("instance", &self.instance)
            .field("surface_khr", &self.surface_khr)
            .field("physical_device", &self.physical_device)
            .finish()
    }
}
