mod debug;

use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{bail, Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// The logical Vulkan instance.
///
/// The instance contains the ash library entry, instance, and any associated
/// debugging information. Within the scope of this library, the Instance is
/// expected to outlive all other Vulkan resources e.g. it should only be
/// dropped once all other resources have been destroyed or dropped.
pub struct Instance {
    pub ash: Arc<raii::Instance>,
    extensions: Vec<String>,
    _debug_utils: Option<Arc<raii::DebugUtils>>,
}

impl Instance {
    /// Create a new Vulkan instance for the given GLFW window.
    pub fn for_window(
        app_name: impl AsRef<str>,
        window: &glfw::Window,
    ) -> Result<Self> {
        if !window.glfw.vulkan_supported() {
            bail!(trace!("Vulkan not supported on this platform!")());
        }

        let extensions = window
            .glfw
            .get_required_instance_extensions()
            .with_context(trace!(
                "Unable to get required extensions for Vulkan instance!"
            ))?;

        Self::new(app_name, &extensions)
    }

    /// Create a new Vulkan instance.
    pub fn new(
        app_name: impl AsRef<str>,
        extensions: &[String],
    ) -> Result<Self> {
        let mut cstrs = extensions
            .iter()
            .cloned()
            .map(|str| std::ffi::CString::new(str).unwrap())
            .collect::<Vec<std::ffi::CString>>();

        if cfg!(debug_assertions) {
            cstrs.push(std::ffi::CString::new("VK_EXT_debug_utils").unwrap());
        }

        let ptrs = cstrs
            .iter()
            .map(|cstr| cstr.as_ptr())
            .collect::<Vec<*const i8>>();

        let app_name_c = std::ffi::CString::new(app_name.as_ref()).unwrap();
        let engine_name = std::ffi::CString::new("N/A").unwrap();
        let application_info = vk::ApplicationInfo {
            p_application_name: app_name_c.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 3, 0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &application_info,
            enabled_extension_count: ptrs.len() as u32,
            pp_enabled_extension_names: ptrs.as_ptr(),
            ..Default::default()
        };

        let ash = raii::Instance::new(&create_info)
            .with_context(trace!("Unable to create instance!"))?;

        let debug_utils = debug::setup_debug_logging(ash.clone())
            .with_context(trace!("Unable to setup debug logging!"))?;

        Ok(Self {
            ash,
            extensions: extensions.to_vec(),
            _debug_utils: debug_utils,
        })
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("extensions", &self.extensions)
            .field("ash", &self.ash)
            .finish()
    }
}
