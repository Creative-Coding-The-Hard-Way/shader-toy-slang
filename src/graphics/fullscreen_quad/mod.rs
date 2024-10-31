mod descriptors;
mod pipeline;
mod uniform_buffer;

use {
    self::uniform_buffer::UniformBuffer,
    crate::{
        graphics::{
            vulkan::{raii, Device, Frame, FramesInFlight, Swapchain},
            Texture,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    bon::bon,
    std::sync::Arc,
};

/// Render a single fullscreen quad using the provided fragment shader.
///
/// A fullscreen quad will always pass a copy of FrameDataT to the fragment
/// shader with a uniform buffer bound at 0,0.
pub struct FullscreenQuad<FrameDataT: Sized + Copy + Default> {
    pipeline: raii::Pipeline,
    pipeline_layout: raii::PipelineLayout,
    descriptor_set_layout: raii::DescriptorSetLayout,
    _descriptor_pool: raii::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
    uniform_buffer: UniformBuffer<FrameDataT>,
    texture: Option<Texture>,
    sampler: raii::Sampler,
    device: Arc<Device>,
}

#[bon]
impl<FrameDataT> FullscreenQuad<FrameDataT>
where
    FrameDataT: Sized + Copy + Default,
{
    /// Creates a new fullscreen quad that uses the provided fragment shader
    /// source.
    #[builder]
    pub fn new(
        device: Arc<Device>,
        fragment_shader_source: &[u8],
        frames_in_flight: &FramesInFlight,
        swapchain: &Swapchain,
        render_pass: &raii::RenderPass,
        texture: Option<Texture>,
    ) -> Result<Self> {
        let sampler = raii::Sampler::new(
            device.logical_device.clone(),
            &vk::SamplerCreateInfo {
                mag_filter: vk::Filter::LINEAR,
                min_filter: vk::Filter::LINEAR,
                mipmap_mode: vk::SamplerMipmapMode::NEAREST,
                address_mode_u: vk::SamplerAddressMode::REPEAT,
                address_mode_v: vk::SamplerAddressMode::REPEAT,
                address_mode_w: vk::SamplerAddressMode::REPEAT,
                mip_lod_bias: 0.0,
                anisotropy_enable: vk::FALSE,
                max_anisotropy: 0.0,
                compare_enable: vk::FALSE,
                compare_op: vk::CompareOp::ALWAYS,
                min_lod: 0.0,
                max_lod: 0.0,
                border_color: vk::BorderColor::FLOAT_OPAQUE_BLACK,
                unnormalized_coordinates: vk::FALSE,
                ..Default::default()
            },
        )
        .with_context(trace!("Unable to create sampler!"))?;

        let uniform_buffer = UniformBuffer::<FrameDataT>::allocate(
            &device,
            frames_in_flight.frame_count(),
        )
        .with_context(trace!(
            "Error allocating a uniform buffer for per-frame data!"
        ))?;

        let descriptor_set_layout =
            descriptors::create_descriptor_set_layout(&device, &texture)
                .with_context(trace!(
                    "Error while creating the descriptor set layout!"
                ))?;

        let descriptor_pool = descriptors::create_descriptor_pool(
            &device,
            frames_in_flight.frame_count(),
            &texture,
        )
        .with_context(trace!("Error while creating the descriptor pool!"))?;

        let descriptor_sets = descriptors::allocate_descriptor_sets(
            &device,
            &descriptor_pool,
            &descriptor_set_layout,
            frames_in_flight.frame_count(),
        )
        .with_context(trace!("Error while allocating descriptor sets!"))?;

        descriptors::initialize_descriptor_sets(
            &device,
            &descriptor_sets,
            &uniform_buffer,
            &texture,
            &sampler,
        );

        let (pipeline, pipeline_layout) = pipeline::create_pipeline(
            &device,
            swapchain,
            render_pass,
            &descriptor_set_layout,
            fragment_shader_source,
        )?;

        Ok(Self {
            pipeline,
            pipeline_layout,
            descriptor_set_layout,
            _descriptor_pool: descriptor_pool,
            descriptor_sets,
            uniform_buffer,
            device,
            texture,
            sampler,
        })
    }

    /// Rebuilds the graphics pipeline using the provided fragment shader
    /// source.
    ///
    /// # WARNING
    ///
    /// The caller must ensure that the fullscreen quad is not in-use by any
    /// pending frames.
    pub fn rebuild_pipeline(
        &mut self,
        swapchain: &Swapchain,
        render_pass: &raii::RenderPass,
        fragment_shader_source: &[u8],
    ) -> Result<()> {
        let (pipeline, pipeline_layout) = pipeline::create_pipeline(
            &self.device,
            swapchain,
            render_pass,
            &self.descriptor_set_layout,
            fragment_shader_source,
        )
        .with_context(trace!(
            "Error while rebuilding the graphics pipeline!"
        ))?;
        self.pipeline = pipeline;
        self.pipeline_layout = pipeline_layout;
        Ok(())
    }

    /// Update the frame's data and add draw commands to the command buffer.
    ///
    /// The caller must begin a compatible render pass in the command buffer
    /// prior to calling this function.
    pub fn draw(&mut self, frame: &Frame, data: FrameDataT) -> Result<()> {
        // SAFE BECAUSE:
        // The only way to construct a Frame is to get it from the
        // frames-in-flight which ensures that all graphics commands for
        // this frame are complete
        unsafe {
            self.uniform_buffer
                .write_indexed(frame.frame_index(), data)
                .with_context(trace!("Error while updating frame data!"))?;

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
            self.device.cmd_draw(frame.command_buffer(), 6, 1, 0, 0);
        }

        Ok(())
    }
}
