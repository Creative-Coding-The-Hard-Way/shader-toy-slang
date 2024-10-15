mod instance;
mod physical_device;

use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

pub use self::instance::Instance;

/// The Vulkan device is the logical handle for all Vulkan resource operations.
pub struct Device {
    pub instance: Instance,
    pub surface_khr: Arc<raii::Surface>,
    pub physical_device: vk::PhysicalDevice,
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

        Ok(Self {
            instance,
            surface_khr,
            physical_device,
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
