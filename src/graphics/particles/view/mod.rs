mod descriptors;
mod pipeline;

use {
    crate::{
        graphics::vulkan::{raii, Device, FramesInFlight, UniformBuffer},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

#[derive(Debug)]
pub struct ParticlesView {
    projection: UniformBuffer<[f32; 16]>,
    descriptor_sets: Vec<vk::DescriptorSet>,
    descriptor_set_layout: raii::DescriptorSetLayout,
    descriptor_pool: raii::DescriptorPool,
    // pipeline_layout: raii::PipelineLayout,
    // pipeline: raii::Pipeline,
    device: Arc<Device>,
}

impl ParticlesView {
    pub fn new(
        device: Arc<Device>,
        frames_in_flight: &FramesInFlight,
    ) -> Result<Self> {
        let descriptor_set_layout = descriptors::create_descriptor_set_layout(
            device.logical_device.clone(),
        )
        .with_context(trace!("Unable to create view descriptor set layout!"))?;

        let descriptor_pool = descriptors::create_descriptor_pool(
            device.logical_device.clone(),
            frames_in_flight,
        )
        .with_context(trace!("Unable to create view descriptor pool!"))?;

        let descriptor_sets = descriptors::allocate_descriptor_sets(
            &device,
            &descriptor_pool,
            &descriptor_set_layout,
            frames_in_flight,
        )
        .with_context(trace!("Unable to allocate view descriptor sets!"))?;

        let projection = UniformBuffer::allocate(&device, frames_in_flight)
            .with_context(trace!(
                "Unable to allocate a uniform buffer for projection matrix!"
            ))?;

        descriptors::update_descriptor_sets(
            &device,
            &descriptor_sets,
            &projection,
        )
        .with_context(trace!("Error while updating descriptor sets!"))?;

        Ok(Self {
            projection,
            descriptor_sets,
            descriptor_set_layout,
            descriptor_pool,
            device,
        })
    }
}
