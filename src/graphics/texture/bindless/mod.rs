use {
    super::Texture,
    crate::graphics::vulkan::{raii, VulkanContext},
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
/// shader code for descriptor set `n`:
///
/// 1. `[vk_binding(0, n)] SamplerState samplers[3];`
/// 2. `[vk_binding(1, n)] texture2D textures[];`
///
/// Samplers have the following properties (by index):
///
/// 1. `samplers[0]` - Linear filtering, linear mipmaps, no anisotropic
///    filtering
/// 2. `samplers[1]` - Linear filtering, linear mipmaps, with anisotropic
///    filtering
/// 3. `samplers[2]` - Nearest filtering, nearest mipmap, no anisotropic
///    filtering
pub struct BindlessTextureAtlas {
    samplers: Vec<raii::Sampler>,
    // textures: Vec<Arc<Texture>>,
    // descriptors: Vec<vk::DescriptorSet>,
    // descriptor_pool: raii::DescriptorPool,
    descriptor_layout: raii::DescriptorSetLayout,
}

impl BindlessTextureAtlas {
    /// Creates a new atlas that can hold up to max_textures total textures.
    pub fn new(ctx: Arc<VulkanContext>, max_textures: u32) -> Result<Self> {
        let samplers = Self::create_samplers(&ctx)?;
        let descriptor_layout = descriptors::create_descriptor_layout(
            &ctx,
            max_textures,
            &samplers,
        )?;
        Ok(Self {
            samplers,
            descriptor_layout,
        })
    }

    // Private API ------------------------------------------------------------

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
