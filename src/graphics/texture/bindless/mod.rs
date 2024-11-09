use {
    super::Texture,
    crate::graphics::vulkan::{raii, Frame, FramesInFlight, VulkanContext},
    anyhow::Result,
    ash::vk,
    std::sync::Arc,
};

mod descriptors;

/// Maintains descriptors for a list of texture images. Textures can be accessed
/// by their index in shaders when the descriptor is bound. e.g. "bindless"
/// textures.
///
/// The descriptor set provided by the atlas expects the following bindings in
/// shader code for descriptor set 0:
///
/// 1. `[vk_binding(0, 0)] SamplerState samplers[3];`
/// 2. `[vk_binding(1, 0)] texture2D textures[];`
///
/// Samplers have the following properties (by index):
///
/// 1. `samplers[0]` - Linear filtering, linear mipmaps, no anisotropic
///    filtering
/// 2. `samplers[1]` - Linear filtering, linear mipmaps, with anisotropic
///    filtering
/// 3. `samplers[2]` - Nearest filtering, nearest mipmap, no anisotropic
///    filtering
///
/// ## Why Descriptor Set 0?
///
/// Because the BindlessTextureAtlas can be bound a single time at the beginning
/// of the renderpass for the frame and ignored after that. If a higher index is
/// used, then switching to different graphics pipelines could require
/// rebinding.
///
/// This works only so long as any subsequently bound pipelines have a
/// compatible layout (e.g. they use the BindlessTextureAtlas's
/// descriptor_set_layout as descriptor 0).
pub struct BindlessTextureAtlas {
    _samplers: Vec<raii::Sampler>,
    textures: Vec<Arc<Texture>>,
    frame_resources: Vec<FrameResources>,
    pipeline_layout: raii::PipelineLayout,
    descriptor_set_layout: raii::DescriptorSetLayout,
    _descriptor_pool: raii::DescriptorPool,
    ctx: Arc<VulkanContext>,
}

impl BindlessTextureAtlas {
    /// Creates a new atlas that can hold up to max_textures total textures.
    pub fn new(
        ctx: Arc<VulkanContext>,
        max_textures: u32,
        frames_in_flight: &FramesInFlight,
    ) -> Result<Self> {
        let samplers = Self::create_samplers(&ctx)?;
        let descriptor_set_layout = descriptors::create_descriptor_layout(
            &ctx,
            max_textures,
            &samplers,
        )?;
        let pipeline_layout =
            Self::create_pipeline_layout(&ctx, &descriptor_set_layout)?;
        let descriptor_pool = descriptors::create_descriptor_pool(
            &ctx,
            frames_in_flight.frame_count() as u32,
            max_textures,
        )?;
        let descriptor_sets = descriptors::allocate_descriptor_sets(
            &ctx,
            &descriptor_pool,
            &descriptor_set_layout,
            frames_in_flight.frame_count() as u32,
        )?;
        let frame_resources = descriptor_sets
            .iter()
            .copied()
            .map(FrameResources::new)
            .collect::<Vec<_>>();
        Ok(Self {
            _samplers: samplers,
            textures: Vec::with_capacity(max_textures as usize),
            frame_resources,
            pipeline_layout,
            descriptor_set_layout,
            _descriptor_pool: descriptor_pool,
            ctx,
        })
    }

    /// Adds a texture to the atlas. The returned texture number can be used to
    /// index in to the textures array in a shader.
    ///
    /// This function is safe to call at any time. The new texture will be
    /// available in the shader in the next frame.
    pub fn add_texture(&mut self, texture: Arc<Texture>) -> u32 {
        self.textures.push(texture);
        self.textures.len() as u32 - 1
    }

    /// Borrows the descriptor set layout for the atlas. Useful when creating
    /// pipelines.
    pub fn descriptor_set_layout(&self) -> &raii::DescriptorSetLayout {
        &self.descriptor_set_layout
    }

    /// Binds the descriptor for the given frame.
    ///
    /// Conditionally updates the descriptor if any new textures have been added
    /// since the last frame.
    pub fn bind_frame_descriptor(&mut self, frame: &Frame) -> Result<()> {
        let frame_resources = &mut self.frame_resources[frame.frame_index()];
        if frame_resources.needs_update(&self.textures) {
            frame_resources.update_descriptor_set(&self.ctx, &self.textures)?;
        }
        unsafe {
            self.ctx.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                0,
                &[frame_resources.descriptor_set],
                &[],
            );
        }
        Ok(())
    }

    // Private API ------------------------------------------------------------

    fn create_pipeline_layout(
        ctx: &VulkanContext,
        descriptor_set_layout: &raii::DescriptorSetLayout,
    ) -> Result<raii::PipelineLayout> {
        raii::PipelineLayout::new(
            ctx.device.clone(),
            &vk::PipelineLayoutCreateInfo {
                set_layout_count: 1,
                p_set_layouts: &descriptor_set_layout.raw,
                push_constant_range_count: 0,
                p_push_constant_ranges: std::ptr::null(),
                ..Default::default()
            },
        )
    }

    fn create_samplers(ctx: &VulkanContext) -> Result<Vec<raii::Sampler>> {
        // find out the max anisotropy:
        let properties = unsafe {
            ctx.instance
                .get_physical_device_properties(ctx.physical_device)
        };

        Ok(vec![
            // Linear Sampler - no anisotropy
            raii::Sampler::new(
                ctx.device.clone(),
                &vk::SamplerCreateInfo {
                    mag_filter: vk::Filter::LINEAR,
                    min_filter: vk::Filter::LINEAR,
                    mipmap_mode: vk::SamplerMipmapMode::LINEAR,
                    address_mode_u: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    address_mode_v: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    address_mode_w: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    mip_lod_bias: 0.0,
                    anisotropy_enable: vk::FALSE,
                    compare_enable: vk::FALSE,
                    min_lod: 0.0,
                    max_lod: vk::LOD_CLAMP_NONE,
                    border_color: vk::BorderColor::INT_OPAQUE_BLACK,
                    unnormalized_coordinates: vk::FALSE,
                    ..Default::default()
                },
            )?,
            // Linear Sampler - with anisotropic sampling
            raii::Sampler::new(
                ctx.device.clone(),
                &vk::SamplerCreateInfo {
                    mag_filter: vk::Filter::LINEAR,
                    min_filter: vk::Filter::LINEAR,
                    mipmap_mode: vk::SamplerMipmapMode::LINEAR,
                    address_mode_u: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    address_mode_v: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    address_mode_w: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    mip_lod_bias: 0.0,
                    anisotropy_enable: vk::TRUE,
                    max_anisotropy: properties.limits.max_sampler_anisotropy,
                    compare_enable: vk::FALSE,
                    min_lod: 0.0,
                    max_lod: vk::LOD_CLAMP_NONE,
                    border_color: vk::BorderColor::INT_OPAQUE_BLACK,
                    unnormalized_coordinates: vk::FALSE,
                    ..Default::default()
                },
            )?,
            // Nearest Sampler
            raii::Sampler::new(
                ctx.device.clone(),
                &vk::SamplerCreateInfo {
                    mag_filter: vk::Filter::NEAREST,
                    min_filter: vk::Filter::NEAREST,
                    mipmap_mode: vk::SamplerMipmapMode::NEAREST,
                    address_mode_u: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    address_mode_v: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    address_mode_w: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                    mip_lod_bias: 0.0,
                    anisotropy_enable: vk::FALSE,
                    compare_enable: vk::FALSE,
                    min_lod: 0.0,
                    max_lod: vk::LOD_CLAMP_NONE,
                    border_color: vk::BorderColor::INT_OPAQUE_BLACK,
                    unnormalized_coordinates: vk::FALSE,
                    ..Default::default()
                },
            )?,
        ])
    }
}

/// The BindlessTextureAtlas's per-Frame resources.
struct FrameResources {
    descriptor_set: vk::DescriptorSet,
    total_bound_textures: usize,
}

impl FrameResources {
    pub fn new(descriptor_set: vk::DescriptorSet) -> Self {
        Self {
            descriptor_set,
            total_bound_textures: 0,
        }
    }

    pub fn update_descriptor_set(
        &self,
        ctx: &VulkanContext,
        textures: &[Arc<Texture>],
    ) -> Result<()> {
        let image_infos = textures
            .iter()
            .map(|texture| vk::DescriptorImageInfo {
                sampler: vk::Sampler::null(),
                image_view: texture.view(),
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            })
            .collect::<Vec<_>>();
        let writes = [vk::WriteDescriptorSet {
            dst_set: self.descriptor_set,
            dst_binding: 1,
            dst_array_element: 0,
            descriptor_count: textures.len() as u32,
            descriptor_type: vk::DescriptorType::SAMPLED_IMAGE,
            p_image_info: image_infos.as_ptr(),
            p_buffer_info: std::ptr::null(),
            p_texel_buffer_view: std::ptr::null(),
            ..Default::default()
        }];
        unsafe {
            ctx.update_descriptor_sets(&writes, &[]);
        }
        Ok(())
    }

    pub fn needs_update(&self, textures: &[Arc<Texture>]) -> bool {
        self.total_bound_textures < textures.len()
    }
}
