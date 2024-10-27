mod allocation_requirements;
pub mod block;
mod dedicated_allocator;
pub mod owned_block;

use {
    self::allocation_requirements::AllocationRequirements,
    crate::{
        graphics::vulkan::{raii, Block},
        trace,
    },
    anyhow::{bail, Context, Result},
    ash::vk,
    std::{
        sync::{
            mpsc::{Sender, SyncSender},
            Arc,
        },
        thread::JoinHandle,
    },
};

trait ManagedAllocator {
    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block>;

    fn free_memory(&mut self, block: &Block);
}

struct SystemAllocator {
    logical_device: Arc<raii::Device>,
}

impl ManagedAllocator for SystemAllocator {
    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        // Allocate the underlying memory
        let memory = unsafe {
            self.logical_device
                .allocate_memory(&requirements.as_vk_allocate_info(), None)
                .with_context(trace!("Unable to allocate device memory!"))?
        };

        // Map the device memory if possible
        let mapped_ptr = if requirements
            .flags
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            unsafe {
                self.logical_device
                    .map_memory(
                        memory,
                        0,
                        vk::WHOLE_SIZE,
                        vk::MemoryMapFlags::empty(),
                    )
                    .with_context(trace!("Unable to map memory!"))?
            }
        } else {
            std::ptr::null_mut()
        };

        Ok(Block::new(
            0,
            requirements.allocation_size,
            memory,
            mapped_ptr,
        ))
    }

    fn free_memory(&mut self, block: &Block) {
        unsafe {
            self.logical_device.free_memory(block.memory(), None);
        }
    }
}

enum Request {
    Allocate(AllocationRequirements, SyncSender<Result<Block>>),
    Free(Block),
    ShutDown,
}

/// A Vulkan device memory allocator.
///
/// # Ownership
///
/// The owner of the Allocator is responsible for ensuring that the phisacal
/// device, the ash library instance, and the Vulkan logical device all outlive
/// the Allocator.
pub struct Allocator {
    logical_device: Arc<raii::Device>,
    client: Sender<Request>,
    allocation_thread: Option<JoinHandle<()>>,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
}

impl Allocator {
    pub fn new(
        logical_device: Arc<raii::Device>,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let memory_properties = unsafe {
            logical_device
                .instance
                .get_physical_device_memory_properties(physical_device)
        };

        let logical_device_clone = logical_device.clone();
        let (sender, receiver) = std::sync::mpsc::channel::<Request>();
        let handle = std::thread::spawn(move || {
            let mut allocator = SystemAllocator {
                logical_device: logical_device_clone,
            };
            'main: loop {
                let allocation_request = if let Ok(request) = receiver.recv() {
                    request
                } else {
                    log::warn!("Memory allocation client hung up!");
                    break 'main;
                };

                match allocation_request {
                    Request::Allocate(requirements, response) => {
                        let result = allocator.allocate_memory(requirements);
                        if let Err(error) = response.send(result) {
                            log::error!(
                                "Unable to send block to requester! {}",
                                error
                            );
                            break 'main;
                        }
                    }
                    Request::Free(block) => {
                        allocator.free_memory(&block);
                    }
                    Request::ShutDown => {
                        log::trace!("Shutdown requested");
                        break 'main;
                    }
                }
            }
            log::trace!("Device memory allocator shut down.");
        });

        Ok(Self {
            logical_device,
            client: sender,
            allocation_thread: Some(handle),
            memory_properties,
        })
    }

    /// Allocates device memory according to the given requirements.
    pub fn allocate_memory(
        &self,
        requirements: &vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<Block> {
        let requirements = AllocationRequirements::new(
            &self.memory_properties,
            requirements,
            flags,
        )?;

        // Send the memory allocation request to the allocator thread
        let (response_sender, response) =
            std::sync::mpsc::sync_channel::<Result<Block>>(1);
        if self
            .client
            .send(Request::Allocate(requirements, response_sender))
            .is_err()
        {
            bail!(trace!("Unable to send allocation request!")());
        }

        // wait for the response
        response
            .recv()
            .with_context(trace!("Error while receiving response!"))?
    }

    /// Free the allocated block.
    pub fn free(&self, block: &Block) {
        if self.client.send(Request::Free(*block)).is_err() {
            log::error!("Error while attempting to free memory: {:#?}", block);
        }
    }
}

impl std::fmt::Debug for Allocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Allocator").finish_non_exhaustive()
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        if self.client.send(Request::ShutDown).is_err() {
            log::error!("Error while sending shutdown request!");
        }
        let allocator_thread_result =
            self.allocation_thread.take().unwrap().join();
        if let Err(error) = allocator_thread_result {
            log::error!("Error in allocator thread!\n\n{:?}", error);
        }
    }
}
