mod descriptors;
mod frame_resources;
mod pipeline;
mod sprite_batch;

use {
    self::frame_resources::FrameResources,
    super::vulkan::Frame,
    crate::graphics::vulkan::{
        raii, DescriptorBumpAllocator, FramesInFlight, Swapchain,
        UniformBuffer, VulkanContext,
    },
    anyhow::Result,
    ash::vk,
    bon::bon,
    nalgebra::Matrix4,
    std::sync::Arc,
};

pub use self::sprite_batch::{SpriteBatch, StreamingSprites};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Sprite {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub uv_pos: [f32; 2],
    pub uv_size: [f32; 2],
    pub tint: [f32; 4],
    pub angle: f32,
    pub texture: i32,
    pub sampler: u32,
    pub padding: f32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0],
            size: [1.0, 1.0],
            uv_pos: [0.0, 0.0],
            uv_size: [1.0, 1.0],
            tint: [1.0, 1.0, 1.0, 1.0],
            angle: 0.0,
            texture: -1,
            sampler: 0,
            padding: Default::default(),
        }
    }
}

/// Layer data provided to the graphics pipeline.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct LayerData {
    pub projection: [[f32; 4]; 4],
}

pub struct SpriteLayerCommands<'a, 'b>(&'a mut SpriteLayer, &'b Frame);

impl SpriteLayerCommands<'_, '_> {
    /// Draw a batch of sprites.
    pub fn draw(self, batch: &impl SpriteBatch) -> Result<Self> {
        let Self(layer, frame) = self;
        layer.draw_batch(frame, batch)?;
        Ok(Self(layer, frame))
    }

    /// Finishes rendering the sprite layer on the frame.
    pub fn finish(self) {
        // no-op
    }
}

/// A pipeline + resources for rendering sprites.
pub struct SpriteLayer {
    layer_descriptor_set_layout: raii::DescriptorSetLayout,
    batch_descriptor_set_layout: raii::DescriptorSetLayout,

    frames: Vec<FrameResources>,

    descriptor_allocator: DescriptorBumpAllocator,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
    layer_data_buffer: UniformBuffer<LayerData>,
    current_layer_data: LayerData,
    ctx: Arc<VulkanContext>,
}

#[bon]
impl SpriteLayer {
    /// Creates a new sprite layer for use with the provided render pass.
    #[builder]
    pub fn new(
        ctx: Arc<VulkanContext>,
        frames_in_flight: &FramesInFlight,
        render_pass: &raii::RenderPass,
        swapchain: &Swapchain,
        projection: Matrix4<f32>,
        texture_atlas_layout: &raii::DescriptorSetLayout,
    ) -> Result<Self> {
        let layer_descriptor_set_layout =
            descriptors::create_layer_descriptor_set_layout(&ctx)?;
        let batch_descriptor_set_layout =
            descriptors::create_batch_descriptor_set_layout(&ctx)?;
        let mut descriptor_allocator =
            descriptors::create_descriptor_allocator(ctx.clone())?;

        let pipeline_layout = pipeline::create_pipeline_layout(
            &ctx,
            &[
                texture_atlas_layout,
                &layer_descriptor_set_layout,
                &batch_descriptor_set_layout,
            ],
        )?;
        let pipeline = pipeline::create_pipeline(
            &ctx,
            &pipeline_layout,
            render_pass,
            swapchain,
        )?;

        let layer_data_buffer =
            UniformBuffer::allocate_per_frame(&ctx, frames_in_flight)?;

        let current_layer_data = LayerData {
            projection: projection.data.0,
        };

        let mut frames = vec![];
        for index in 0..frames_in_flight.frame_count() {
            frames.push(FrameResources::new(
                ctx.clone(),
                &mut descriptor_allocator,
                &layer_descriptor_set_layout,
                &layer_data_buffer,
                index,
            )?);
        }

        Ok(Self {
            layer_descriptor_set_layout,
            batch_descriptor_set_layout,
            frames,
            descriptor_allocator,
            pipeline_layout,
            pipeline,
            layer_data_buffer,
            current_layer_data,
            ctx,
        })
    }

    /// Begin rendering a layer to the frame.
    pub fn begin_frame_commands<'a, 'b>(
        &'a mut self,
        frame: &'b Frame,
    ) -> Result<SpriteLayerCommands<'a, 'b>> {
        self.bind_pipeline(frame)?;
        Ok(SpriteLayerCommands(self, frame))
    }

    /// Reset the Sprite Layer's internal resources.
    ///
    /// This can be more efficient than destroying and recreating a new sprite
    /// layer.
    ///
    /// # Performance
    ///
    /// This method waits for all pending frames in flight to complete.
    pub fn reset(&mut self, frames_in_flight: &FramesInFlight) -> Result<()> {
        frames_in_flight.wait_for_all_frames_to_complete()?;
        self.frames.clear();
        unsafe {
            // SAFE: because there are no pending frames-in-flight
            self.descriptor_allocator.reset()?;
        }

        for index in 0..frames_in_flight.frame_count() {
            self.frames.push(FrameResources::new(
                self.ctx.clone(),
                &mut self.descriptor_allocator,
                &self.layer_descriptor_set_layout,
                &self.layer_data_buffer,
                index,
            )?);
        }

        Ok(())
    }

    /// Update any swapchain-dependent internal resources.
    ///
    /// # Performance
    ///
    /// Note, this method waits for all pending frames in flight to complete.
    pub fn rebuild_swapchain_resources(
        &mut self,
        swapchain: &Swapchain,
        renderpass: &raii::RenderPass,
        frames_in_flight: &FramesInFlight,
    ) -> Result<()> {
        frames_in_flight.wait_for_all_frames_to_complete()?;

        self.pipeline = pipeline::create_pipeline(
            &self.ctx,
            &self.pipeline_layout,
            renderpass,
            swapchain,
        )?;
        Ok(())
    }

    /// Sets the layer's projection matrix. This can be called at any time and
    /// will take effect in the next draw_batches() call.
    pub fn set_projection(&mut self, projection: &Matrix4<f32>) {
        self.current_layer_data.projection = projection.data.0;
    }

    // Private API ------------------------------------------------------------

    /// Binds the pipeline to the frame command buffer.
    fn bind_pipeline(&mut self, frame: &Frame) -> Result<()> {
        self.layer_data_buffer
            .update_frame_data(frame, self.current_layer_data)?;

        let resources = &mut self.frames[frame.frame_index()];
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
                &[resources.get_layer_descriptor()],
                &[],
            );
        }
        Ok(())
    }

    /// Binds a descriptor set for the batch and adds a draw command to the
    /// frame's command buffer. Note: it is only valid to call this method after
    /// a corresponding call to [Self::bind_pipeline].
    fn draw_batch(
        &mut self,
        frame: &Frame,
        batch: &impl SpriteBatch,
    ) -> Result<()> {
        if batch.count() == 0 {
            // no-op for empty batches
            return Ok(());
        }

        let resources = &mut self.frames[frame.frame_index()];
        let batch_descriptor = resources.get_batch_descriptor(
            batch,
            &mut self.descriptor_allocator,
            &self.batch_descriptor_set_layout,
        )?;
        unsafe {
            self.ctx.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                2,
                &[batch_descriptor],
                &[],
            );
            self.ctx
                .cmd_draw(frame.command_buffer(), 6, batch.count(), 0, 0);
        }
        Ok(())
    }
}
