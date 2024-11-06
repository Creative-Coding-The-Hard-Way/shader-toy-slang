mod descriptors;
mod pipeline;

use {
    crate::graphics::{
        particles::Particle,
        vulkan::{raii, CPUBuffer, Device, Frame, UniformBuffer},
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
    device: Arc<Device>,
    pipeline: raii::Pipeline,
    pipeline_layout: raii::PipelineLayout,
    frame_data: UniformBuffer<DataT>,
    current_frame: CurrentFrame,
}

#[bon]
impl<DataT: Copy + Sized> ParticlesCompute<DataT> {
    #[builder]
    pub fn new(
        device: Arc<Device>,
        particles_buffer: &CPUBuffer<Particle>,
        kernel_bytes: &[u8],
    ) -> Result<Self> {
        let frame_data = UniformBuffer::allocate(&device, 2)?;

        let descriptor_set_layout = descriptors::create_descriptor_set_layout(
            device.logical_device.clone(),
        )?;
        let descriptor_pool =
            descriptors::create_descriptor_pool(device.logical_device.clone())?;
        let descriptor_sets = descriptors::allocate_descriptor_sets(
            &device,
            &descriptor_pool,
            &descriptor_set_layout,
        )?;
        descriptors::update_descriptor_sets(
            &device,
            &descriptor_sets,
            particles_buffer,
            &frame_data,
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
            descriptor_sets,
            _descriptor_pool: descriptor_pool,
            _descriptor_set_layout: descriptor_set_layout,
            device,
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
        kernel_source: &[u8],
    ) -> Result<()> {
        self.pipeline = pipeline::create_pipeline(
            self.device.logical_device.clone(),
            &self.pipeline_layout,
            kernel_source,
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
                &[current_descriptor_set],
                &[],
            );
            self.device
                .cmd_dispatch(frame.command_buffer(), 320 / 32, 1, 1);
        }
        Ok(())
    }
}
