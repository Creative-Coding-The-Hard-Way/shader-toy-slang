use {
    crate::trace,
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// A RAII wrapper for the ash library entry and instance that destroys itself
/// when dropped.
pub struct Instance {
    pub entry: ash::Entry,
    pub raw: ash::Instance,
}

pub type InstanceArc = Arc<Instance>;

impl Instance {
    pub fn new(create_info: &vk::InstanceCreateInfo) -> Result<Arc<Self>> {
        let entry = unsafe {
            ash::Entry::load().with_context(trace!(
                "Unable to load the default Vulkan library!"
            ))?
        };
        let raw = unsafe {
            entry
                .create_instance(create_info, None)
                .with_context(trace!("Unable to create the Vulkan instance!"))?
        };
        Ok(Arc::new(Self { entry, raw }))
    }
}

impl std::ops::Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.raw.destroy_instance(None);
        }
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance").finish()
    }
}
