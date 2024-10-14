//! RAII wrappers for Vulkan objects.
//!
//! Wrappers do not track dependencies. The application is responsible for
//! dropping Vulkan objects in the correct order and synchronizing to prevent
//! GPU inconsistencies.

mod instance;
mod instance_extensions;

pub use self::{
    instance::{Instance, InstanceArc},
    instance_extensions::{DebugUtils, DebugUtilsArc},
};
