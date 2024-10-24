//! RAII wrappers for Vulkan objects that extend the Vulkan instance.

use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

macro_rules! instance_extension {
    (
        $name: ident,
        $ext_type: ty,
        $raw_type: ty,
        $destroy: ident
    ) => {
        /// RAII wrapper that destroys itself when Dropped.
        ///
        /// The owner is responsible for dropping Vulkan resources in the
        /// correct order.
        pub struct $name {
            pub ext: $ext_type,
            pub raw: $raw_type,
            pub ash: Arc<raii::Instance>,
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("ext", &"<extension loader>")
                    .field("raw", &self.raw)
                    .field("ash", &self.ash)
                    .finish()
            }
        }

        impl std::ops::Deref for $name {
            type Target = $raw_type;

            fn deref(&self) -> &Self::Target {
                &self.raw
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { self.ext.$destroy(self.raw, None) }
            }
        }
    };
}

instance_extension!(
    DebugUtils,
    ash::ext::debug_utils::Instance,
    vk::DebugUtilsMessengerEXT,
    destroy_debug_utils_messenger
);

impl DebugUtils {
    pub fn new(
        ash: Arc<raii::Instance>,
        create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
    ) -> Result<Arc<Self>> {
        let ext = ash::ext::debug_utils::Instance::new(&ash.entry, &ash);
        let raw =
            unsafe { ext.create_debug_utils_messenger(create_info, None)? };
        Ok(Arc::new(Self { ext, raw, ash }))
    }
}

instance_extension!(
    Surface,
    ash::khr::surface::Instance,
    vk::SurfaceKHR,
    destroy_surface
);

impl Surface {
    pub fn new(
        ash: Arc<raii::Instance>,
        raw: vk::SurfaceKHR,
    ) -> Result<Arc<Self>> {
        let ext = ash::khr::surface::Instance::new(&ash.entry, &ash);
        Ok(Arc::new(Self { raw, ext, ash }))
    }

    pub fn for_window(
        ash: Arc<raii::Instance>,
        window: &glfw::Window,
    ) -> Result<Arc<Self>> {
        let handle = {
            let mut surface = ash::vk::SurfaceKHR::null();
            window
                .create_window_surface(
                    ash.raw.handle(),
                    std::ptr::null(),
                    &mut surface,
                )
                .result()
                .with_context(trace!(
                    "Unable to create the Vulkan SurfaceKHR with GLFW!"
                ))?;
            surface
        };
        Self::new(ash, handle)
    }
}
