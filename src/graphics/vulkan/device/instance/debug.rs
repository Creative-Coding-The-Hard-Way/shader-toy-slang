use {
    crate::graphics::vulkan::raii,
    anyhow::Result,
    ash::vk::{
        self, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCallbackDataEXT,
    },
    std::{borrow::Cow, ffi::CStr, sync::Arc},
};

/// Setup debug logging.
///
/// This is a no-op if the debug_asserts are not enabled.
pub(super) fn setup_debug_logging(
    instance: Arc<raii::Instance>,
) -> Result<Option<Arc<raii::DebugUtils>>> {
    if !cfg!(debug_assertions) {
        return Ok(None);
    }

    let create_info = vk::DebugUtilsMessengerCreateInfoEXT {
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
            | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        pfn_user_callback: Some(debug_callback),
        ..Default::default()
    };
    Ok(Some(raii::DebugUtils::new(instance, &create_info)?))
}

unsafe extern "system" fn debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message_number = callback_data.message_id_number;
    if message_number == 0 {
        return vk::FALSE;
    }

    let raw_message = std::format!(
        indoc::indoc! {
            "
            Vulkan Debug Callback

            - Severity: {:?}::{:?}
            - Message ID: {}

            {}
            "
        },
        message_severity,
        message_type,
        message_id_name,
        message
    );

    let full_message = raw_message.replace("; ", ";\n\n");

    match message_severity {
        DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            log::trace!("{}", full_message);
        }

        DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log::trace!("{}", full_message);
        }

        DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log::warn!("{}", full_message);
        }

        DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log::error!("{}", full_message);
        }

        _ => {
            log::warn!("?? {}", full_message);
        }
    }

    vk::FALSE
}
