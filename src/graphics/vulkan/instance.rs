use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk,
};

pub struct Instance {
    pub ash: raii::InstanceArc,
    extensions: Vec<String>,
}

impl Instance {
    /// Create a new Vulkan instance.
    pub fn new<S>(app_name: S, extensions: &[String]) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let cstrs = extensions
            .iter()
            .cloned()
            .map(|str| std::ffi::CString::new(str).unwrap())
            .collect::<Vec<std::ffi::CString>>();
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

        Ok(Self {
            ash,
            extensions: extensions.to_vec(),
        })
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("extensions", &self.extensions)
            .finish()
    }
}
