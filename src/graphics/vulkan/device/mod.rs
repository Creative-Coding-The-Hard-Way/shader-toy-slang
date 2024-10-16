mod instance;
mod logical_device;
mod physical_device;

use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk::{self},
    std::sync::Arc,
};

pub use self::instance::Instance;

/// The Vulkan device is the logical handle for all Vulkan resource operations.
pub struct Device {
    pub instance: Instance,
    pub surface_khr: Arc<raii::Surface>,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Arc<raii::Device>,

    /// The queue family index for the graphics + present queue.
    pub graphics_queue_family_index: u32,

    /// The graphics queue supports GRAPHICS and presentation operations.
    pub graphics_queue: vk::Queue,
}

impl Device {
    pub fn new(window: &glfw::Window) -> Result<Self> {
        let instance = Instance::for_window("Shader-Toy-Slang", window)
            .with_context(trace!("Unable to create vulkan instance!"))?;

        let surface_khr =
            raii::Surface::for_window(instance.ash.clone(), window)
                .with_context(trace!(
                    "Unable to create Vulkan surface from glfw window!"
                ))?;

        let physical_device =
            physical_device::pick_suitable_device(&instance, &surface_khr)
                .with_context(trace!(
                    "Error while picking a suitable physical device!"
                ))?;

        let (logical_device, graphics_queue_family_index) =
            logical_device::create_logical_device(
                &instance,
                &surface_khr,
                physical_device,
            )
            .with_context(trace!("Error while creating the logical device!"))?;

        let graphics_queue = unsafe {
            logical_device.get_device_queue(graphics_queue_family_index, 0)
        };

        Ok(Self {
            instance,
            surface_khr,
            physical_device,
            logical_device,
            graphics_queue_family_index,
            graphics_queue,
        })
    }
}

impl std::ops::Deref for Device {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        &self.logical_device.raw
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("instance", &self.instance)
            .field("surface_khr", &self.surface_khr)
            .field("physical_device", &self.physical_device)
            .field("logical_device", &self.logical_device)
            .field(
                "graphics_queue_family_index",
                &self.graphics_queue_family_index,
            )
            .field("graphics_queue", &self.graphics_queue)
            .finish()
    }
}
