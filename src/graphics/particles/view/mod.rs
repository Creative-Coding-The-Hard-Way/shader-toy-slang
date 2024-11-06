mod descriptors;
mod pipeline;

use {
    super::Particle,
    crate::{
        graphics::{
            ortho_projection,
            vulkan::{
                raii, CPUBuffer, Device, Frame, FramesInFlight, Swapchain,
                UniformBuffer,
            },
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    bon::bon,
    nalgebra::Matrix4,
    std::sync::Arc,
};

#[derive(Debug)]
pub struct ParticlesView {
    projection: UniformBuffer<[[f32; 4]; 4]>,
    descriptor_sets: Vec<vk::DescriptorSet>,
    _descriptor_set_layout: raii::DescriptorSetLayout,
    _descriptor_pool: raii::DescriptorPool,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
    projection_matrix: Matrix4<f32>,
    device: Arc<Device>,
}

#[bon]
impl ParticlesView {
    #[builder]
    pub fn new(
        device: Arc<Device>,
        frames_in_flight: &FramesInFlight,
        swapchain: &Swapchain,
        render_pass: &raii::RenderPass,
        particles_buffer: &CPUBuffer<Particle>,
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

        let projection =
            UniformBuffer::allocate_per_frame(&device, frames_in_flight)
                .with_context(trace!(
                "Unable to allocate a uniform buffer for projection matrix!"
            ))?;

        descriptors::update_descriptor_sets(
            &device,
            &descriptor_sets,
            &projection,
            particles_buffer,
        )
        .with_context(trace!("Error while updating descriptor sets!"))?;

        let pipeline_layout = pipeline::create_layout(
            device.logical_device.clone(),
            &descriptor_set_layout,
        )
        .with_context(trace!("Unable to create the pipeline layout!"))?;

        let pipeline = pipeline::create_pipeline(
            device.logical_device.clone(),
            swapchain,
            render_pass,
            &pipeline_layout,
        )
        .with_context(trace!("Unable to create the graphics pipeline!"))?;

        let projection_matrix = ortho_projection(
            swapchain.extent().width as f32 / swapchain.extent().height as f32,
            20.0,
        );

        Ok(Self {
            projection,
            projection_matrix,
            descriptor_sets,
            _descriptor_set_layout: descriptor_set_layout,
            _descriptor_pool: descriptor_pool,
            pipeline_layout,
            pipeline,
            device,
        })
    }

    pub fn draw(&mut self, frame: &Frame) -> Result<()> {
        self.projection
            .update_frame_data(frame, self.projection_matrix.data.0)?;

        unsafe {
            self.device.cmd_bind_pipeline(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.raw,
            );

            self.device.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                0,
                &[self.descriptor_sets[frame.frame_index()]],
                &[],
            );

            self.device.cmd_draw(frame.command_buffer(), 6, 320, 0, 0);
        }

        Ok(())
    }

    pub fn swapchain_rebuilt(
        &mut self,
        swapchain: &Swapchain,
        render_pass: &raii::RenderPass,
    ) -> Result<()> {
        self.pipeline = pipeline::create_pipeline(
            self.device.logical_device.clone(),
            swapchain,
            render_pass,
            &self.pipeline_layout,
        )?;

        self.projection_matrix = ortho_projection(
            swapchain.extent().width as f32 / swapchain.extent().height as f32,
            20.0,
        );
        Ok(())
    }
}
