mod descriptors;
mod pipeline;

use {
    anyhow::Result,
    ash::vk,
    demo_vk::graphics::vulkan::{
        raii, Frame, FramesInFlight, UniformBuffer, VulkanContext,
    },
    std::sync::Arc,
};

// This can be accepted in the fragment shader with code like:
//
//   struct FrameData {
//       float2 mouse_pos;
//       float2 screen_size;
//       float dt;
//       float time;
//   };
//
//   [[vk_binding(0, 1)]] ConstantBuffer<FrameData> frame;
//
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(C)]
pub struct FrameData {
    pub mouse_pos: [f32; 2],
    pub screen_size: [f32; 2],
    pub dt: f32,
    pub time: f32,
}

/// A fullscreen effect is a reloadable fragment shader that is provided with
/// updated data for each frame.
pub struct FullscreenEffect {
    effect_shader: Arc<raii::ShaderModule>,
    frame_data: UniformBuffer<FrameData>,
    descriptor_sets: Vec<vk::DescriptorSet>,
    _descriptor_pool: raii::DescriptorPool,
    _descriptor_set_layout: raii::DescriptorSetLayout,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
    ctx: Arc<VulkanContext>,
}

#[bon::bon]
impl FullscreenEffect {
    #[builder]
    pub fn new(
        ctx: Arc<VulkanContext>,
        frames_in_flight: &FramesInFlight,
        texture_atlas_layout: &raii::DescriptorSetLayout,
        render_pass: &raii::RenderPass,
        effect_shader: Arc<raii::ShaderModule>,
    ) -> Result<Self> {
        let frame_data =
            UniformBuffer::allocate(&ctx, frames_in_flight.frame_count())?;

        let descriptor_set_layout = Self::create_descriptor_set_laout(&ctx)?;
        let descriptor_pool = Self::create_descriptor_pool(
            &ctx,
            frames_in_flight.frame_count() as u32,
        )?;
        let descriptor_sets = Self::allocate_descriptor_sets(
            &ctx,
            &descriptor_pool,
            &descriptor_set_layout,
            frames_in_flight.frame_count() as u32,
        )?;
        Self::update_descriptor_sets(&ctx, &descriptor_sets, &frame_data);

        let pipeline_layout = Self::create_pipeline_layout(
            &ctx,
            &[texture_atlas_layout, &descriptor_set_layout],
        )?;
        let pipeline = Self::create_pipeline(
            &ctx,
            &pipeline_layout,
            render_pass,
            &effect_shader,
        )?;

        Ok(Self {
            effect_shader,
            frame_data,
            descriptor_sets,
            _descriptor_pool: descriptor_pool,
            _descriptor_set_layout: descriptor_set_layout,
            pipeline_layout,
            pipeline,
            ctx,
        })
    }

    /// Draws the fullscreen effect.
    ///
    /// Note: The viewport and scissor state must be set prior to calling this
    /// function and the texture atlas descriptor set must be bound.
    pub fn draw(&mut self, frame: &Frame, data: FrameData) -> Result<()> {
        self.frame_data.update_frame_data(frame, data)?;

        unsafe {
            self.ctx.cmd_bind_pipeline(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.raw,
            );
            self.ctx.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                1,
                &[self.descriptor_sets[frame.frame_index()]],
                &[],
            );
            self.ctx.cmd_draw(frame.command_buffer(), 6, 1, 0, 0);
        }
        Ok(())
    }

    /// Rebuilds th effect pipeline.
    ///
    /// # Params
    ///
    /// * `effect_shader` can either be `None` or `Some(shader)`. If `None` then
    ///   the current effect will be retained.
    ///
    /// # Safety
    ///
    /// This function can only be called after a device wait_idle or after
    /// waiting for all frames_in_flight to finish.
    pub fn rebuild_pipeline(
        &mut self,
        render_pass: &raii::RenderPass,
        effect_shader: Option<Arc<raii::ShaderModule>>,
    ) -> Result<()> {
        if let Some(shader) = effect_shader {
            self.effect_shader = shader;
        }

        self.pipeline = Self::create_pipeline(
            &self.ctx,
            &self.pipeline_layout,
            render_pass,
            &self.effect_shader,
        )?;

        Ok(())
    }
}
