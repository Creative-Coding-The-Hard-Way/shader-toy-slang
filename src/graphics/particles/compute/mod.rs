mod descriptors;
mod pipeline;

use {
    crate::graphics::{
        particles::Particle,
        vulkan::{raii, CPUBuffer, Device, Frame},
    },
    anyhow::Result,
    ash::vk,
    bon::bon,
    std::sync::Arc,
};

/// Resources required to dispatch the Particles compute pipeline.
#[derive(Debug)]
pub struct ParticlesCompute {
    descriptor_set: vk::DescriptorSet,
    descriptor_pool: raii::DescriptorPool,
    descriptor_set_layout: raii::DescriptorSetLayout,
    device: Arc<Device>,
    pipeline: raii::Pipeline,
    pipeline_layout: raii::PipelineLayout,
}

#[bon]
impl ParticlesCompute {
    #[builder]
    pub fn new(
        device: Arc<Device>,
        particles_buffer: &CPUBuffer<Particle>,
        kernel_bytes: &[u8],
    ) -> Result<Self> {
        let descriptor_set_layout = descriptors::create_descriptor_set_layout(
            device.logical_device.clone(),
        )?;
        let descriptor_pool =
            descriptors::create_descriptor_pool(device.logical_device.clone())?;
        let descriptor_set = descriptors::allocate_descriptor_set(
            &device,
            &descriptor_pool,
            &descriptor_set_layout,
        )?;
        descriptors::update_descriptor_set(
            &device,
            descriptor_set,
            particles_buffer,
        )?;

        let pipeline_layout = pipeline::create_layout(
            device.logical_device.clone(),
            &descriptor_set_layout,
        )?;

        let pipeline = pipeline::create_pipeline(
            device.logical_device.clone(),
            &pipeline_layout,
            kernel_bytes,
        )?;

        Ok(Self {
            descriptor_set,
            descriptor_pool,
            descriptor_set_layout,
            device,
            pipeline_layout,
            pipeline,
        })
    }

    /// # Safety
    ///
    /// Unsafe because:
    /// - the caller must ensure that there are no pending compute operations
    ///   when this function is called.
    pub unsafe fn rebuild_kernel(
        &mut self,
        kernel_source: &[u8],
    ) -> Result<()> {
        self.pipeline = pipeline::create_pipeline(
            self.device.logical_device.clone(),
            &self.pipeline_layout,
            kernel_source,
        )?;
        Ok(())
    }

    pub fn update(&self, frame: &Frame) -> Result<()> {
        unsafe {
            self.device.cmd_bind_pipeline(
                frame.command_buffer(),
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline.raw,
            );
            self.device.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline_layout.raw,
                0,
                &[self.descriptor_set],
                &[],
            );
            self.device.cmd_dispatch(frame.command_buffer(), 1, 1, 1);
        }
        Ok(())
    }
}
