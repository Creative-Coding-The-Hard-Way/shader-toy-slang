mod frame_data;
mod pipeline;
mod static_textures;

use {
    self::{frame_data::FrameData, static_textures::StaticTextures},
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
/// A fullscreen quad will always pass a copy of UserDataT to the fragment
/// shader with a uniform buffer bound at 0,0.
pub struct FullscreenQuad<UserDataT: Sized + Copy + Default> {
    pipeline: raii::Pipeline,
    pipeline_layout: raii::PipelineLayout,
    frame_data: FrameData<UserDataT>,
    static_textures: StaticTextures,
    device: Arc<Device>,
}

#[bon]
impl<UserDataT> FullscreenQuad<UserDataT>
where
    UserDataT: Sized + Copy + Default,
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
        textures: Vec<Texture>,
    ) -> Result<Self> {
        let frame_data = FrameData::new(&device, &frames_in_flight)
            .with_context(trace!("Unable to initialize per-frame data!"))?;

        let static_textures = StaticTextures::new(&device, textures)
            .with_context(trace!(
                "Unable to create static textures resources!"
            ))?;

        let (pipeline, pipeline_layout) = pipeline::create_pipeline(
            &device,
            swapchain,
            render_pass,
            &[
                frame_data.descriptor_set_layout(),
                static_textures.descriptor_set_layout(),
            ],
            fragment_shader_source,
        )?;

        Ok(Self {
            pipeline,
            pipeline_layout,
            frame_data,
            static_textures,
            device,
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
            &[
                self.frame_data.descriptor_set_layout(),
                self.static_textures.descriptor_set_layout(),
            ],
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
    pub fn draw(&mut self, frame: &Frame, data: UserDataT) -> Result<()> {
        self.frame_data.update(frame, data).with_context(trace!(
            "Unable to update user data for the frame!"
        ))?;

        // SAFE BECAUSE:
        // The only way to construct a Frame is to get it from the
        // frames-in-flight which ensures that all graphics commands for
        // this frame are complete
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
                &[
                    self.frame_data.current_descriptor_set(frame),
                    self.static_textures.descriptor_set(),
                ],
                &[],
            );
            self.device.cmd_draw(frame.command_buffer(), 6, 1, 0, 0);
        }

        Ok(())
    }
}
