//! This module defines traits, structs, and functions to interact with a Vulkan
//! device.
//!
//! The primary entrypoint is the [VulkanContext] which can be initialized with
//! a [glfw::Window].

mod allocator;
mod buffers;
mod context;
mod frames_in_flight;
pub mod raii;
mod swapchain;
mod sync_commands;

pub use self::{
    allocator::{block::Block, owned_block::OwnedBlock, Allocator},
    buffers::{CPUBuffer, UniformBuffer},
    context::{Instance, VulkanContext},
    frames_in_flight::{Frame, FrameStatus, FramesInFlight},
    swapchain::{AcquireImageStatus, PresentImageStatus, Swapchain},
    sync_commands::SyncCommands,
};
