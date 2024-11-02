mod descriptors;

use {
    crate::{
        graphics::vulkan::{
            raii, Device, Frame, FramesInFlight, UniformBuffer,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// All resources required to support N frames in flight.
pub struct FrameData<UserDataT: Copy + Sized> {
    descriptor_set_layout: raii::DescriptorSetLayout,
    _descriptor_pool: raii::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
    uniform_buffer: UniformBuffer<UserDataT>,
}

impl<UserDataT> FrameData<UserDataT>
where
    UserDataT: Sized + Copy + Default,
{
    pub fn new(
        device: &Device,
        frames_in_flight: &FramesInFlight,
    ) -> Result<Self> {
        let uniform_buffer =
            UniformBuffer::<UserDataT>::allocate(device, frames_in_flight)
                .with_context(trace!(
                    "Error allocating a uniform buffer for per-frame data!"
                ))?;

        let descriptor_set_layout =
            descriptors::create_descriptor_set_layout(device).with_context(
                trace!("Error while creating the descriptor set layout!"),
            )?;

        let descriptor_pool = descriptors::create_descriptor_pool(
            device,
            frames_in_flight.frame_count(),
        )
        .with_context(trace!("Error while creating the descriptor pool!"))?;

        let descriptor_sets = descriptors::allocate_descriptor_sets(
            device,
            &descriptor_pool,
            &descriptor_set_layout,
            frames_in_flight.frame_count(),
        )
        .with_context(trace!("Error while allocating descriptor sets!"))?;

        descriptors::initialize_descriptor_sets(
            device,
            &descriptor_sets,
            &uniform_buffer,
        );

        Ok(Self {
            descriptor_set_layout,
            _descriptor_pool: descriptor_pool,
            descriptor_sets,
            uniform_buffer,
        })
    }

    /// Get the descriptor set layout for the per-frame binding.
    pub fn descriptor_set_layout(&self) -> &raii::DescriptorSetLayout {
        &self.descriptor_set_layout
    }

    /// Update user data for the frame.
    pub fn update(&mut self, frame: &Frame, data: UserDataT) -> Result<()> {
        unsafe {
            // SAFE because the region of the uniform buffer that's being
            // modified cannot be read while a borrow of Frame still exists.
            self.uniform_buffer
                .write_indexed(frame.frame_index(), data)
                .with_context(trace!("Error while updating frame data!"))?;
        }
        Ok(())
    }

    /// Get a non-owning handle to the descriptor set for the provided frame.
    pub fn current_descriptor_set(&self, frame: &Frame) -> vk::DescriptorSet {
        self.descriptor_sets[frame.frame_index()]
    }
}
