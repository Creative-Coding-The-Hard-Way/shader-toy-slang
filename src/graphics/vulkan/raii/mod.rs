//! RAII wrappers for Vulkan objects.
//!
//! Wrappers do not track dependencies. The application is responsible for
//! dropping Vulkan objects in the correct order and synchronizing to prevent
//! GPU inconsistencies.

mod instance;
mod instance_extensions;

use {anyhow::Result, ash::vk, std::sync::Arc};

pub use self::{
    instance::Instance,
    instance_extensions::{DebugUtils, Surface},
};

/// A RAII wrapper for the Vulkan Logical Device.
pub struct Device {
    pub raw: ash::Device,
    pub instance: Arc<Instance>,
}

impl Device {
    pub fn new(
        instance: Arc<Instance>,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
    ) -> Result<Arc<Self>> {
        let raw = unsafe {
            instance
                .raw
                .create_device(physical_device, create_info, None)?
        };
        Ok(Arc::new(Self { raw, instance }))
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.raw.destroy_device(None) }
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("raw", &"<raw vulkan device handle>")
            .field("instance", &self.instance)
            .finish()
    }
}

impl std::ops::Deref for Device {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
