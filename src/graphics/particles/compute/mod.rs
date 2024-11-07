mod descriptors;
mod pipeline;

use {
    crate::graphics::{
        particles::Particle,
        vulkan::{raii, CPUBuffer, Frame, UniformBuffer, VulkanContext},
    },
    anyhow::Result,
    ash::vk,
    bon::bon,
    std::sync::Arc,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CurrentFrame {
    A,
    B,
}

/// Resources required to dispatch the Particles compute pipeline.
#[derive(Debug)]
pub struct ParticlesCompute<DataT: Copy + Sized> {
    descriptor_sets: Vec<vk::DescriptorSet>,
    _descriptor_pool: raii::DescriptorPool,
    _descriptor_set_layout: raii::DescriptorSetLayout,
    cxt: Arc<VulkanContext>,
    pipeline: raii::Pipeline,
    pipeline_layout: raii::PipelineLayout,
    frame_data: UniformBuffer<DataT>,
    current_frame: CurrentFrame,
}

#[bon]
impl<DataT: Copy + Sized> ParticlesCompute<DataT> {
    #[builder]
    pub fn new(
        cxt: Arc<VulkanContext>,
        particles_buffer: &CPUBuffer<Particle>,
        kernel: &raii::ShaderModule,
    ) -> Result<Self> {
        let frame_data = UniformBuffer::allocate(&cxt, 2)?;

        let descriptor_set_layout =
            descriptors::create_descriptor_set_layout(cxt.device.clone())?;
        let descriptor_pool =
            descriptors::create_descriptor_pool(cxt.device.clone())?;
        let descriptor_sets = descriptors::allocate_descriptor_sets(
            &cxt,
            &descriptor_pool,
            &descriptor_set_layout,
        )?;
        descriptors::update_descriptor_sets(
            &cxt,
            &descriptor_sets,
            particles_buffer,
            &frame_data,
        )?;

        let pipeline_layout = pipeline::create_layout(
            cxt.device.clone(),
            &descriptor_set_layout,
        )?;

        let pipeline = pipeline::create_pipeline(
            cxt.device.clone(),
            &pipeline_layout,
            kernel,
        )?;

        Ok(Self {
            descriptor_sets,
            _descriptor_pool: descriptor_pool,
            _descriptor_set_layout: descriptor_set_layout,
            cxt,
            pipeline_layout,
            pipeline,
            frame_data,
            current_frame: CurrentFrame::A,
        })
    }

    /// # Safety
    ///
    /// Unsafe because:
    /// - the caller must ensure that there are no pending compute operations
    ///   when this function is called.
    pub unsafe fn rebuild_kernel(
        &mut self,
        kernel: &raii::ShaderModule,
    ) -> Result<()> {
        self.pipeline = pipeline::create_pipeline(
            self.cxt.device.clone(),
            &self.pipeline_layout,
            kernel,
        )?;
        Ok(())
    }

    pub fn update(&mut self, frame: &Frame, data: DataT) -> Result<()> {
        let current_descriptor_set = match self.current_frame {
            CurrentFrame::A => {
                self.current_frame = CurrentFrame::B;
                unsafe {
                    self.frame_data.write_indexed(0, data)?;
                }
                self.descriptor_sets[0]
            }
            CurrentFrame::B => {
                self.current_frame = CurrentFrame::A;
                unsafe {
                    self.frame_data.write_indexed(1, data)?;
                }
                self.descriptor_sets[1]
            }
        };

        unsafe {
            self.cxt.cmd_bind_pipeline(
                frame.command_buffer(),
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline.raw,
            );
            self.cxt.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline_layout.raw,
                0,
                &[current_descriptor_set],
                &[],
            );
            self.cxt
                .cmd_dispatch(frame.command_buffer(), 320 / 32, 1, 1);
        }
        Ok(())
    }
}
