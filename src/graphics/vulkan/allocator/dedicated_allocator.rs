use {
    crate::graphics::vulkan::{raii, Block},
    std::sync::Arc,
};

/// The Dedicated Allocator is responsible for allocating exclusive chunks of
/// memory from the device.
///
/// Dedicated allocations should typically only be for large blocks that get
/// subdivided.
pub struct DedicatedAllocator {
    logical_device: Arc<raii::Device>,
    allocations: Vec<Block>,
}
