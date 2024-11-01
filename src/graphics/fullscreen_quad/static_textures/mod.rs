mod descriptors;

use {
    crate::{
        graphics::{
            vulkan::{raii, Device},
            Texture,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// All of the resources required to bind textures.
#[derive(Debug)]
pub struct StaticTextures {
    descriptor_set: vk::DescriptorSet,
    _descriptor_pool: raii::DescriptorPool,
    descriptor_set_layout: raii::DescriptorSetLayout,
    _sampler: raii::Sampler,
    _textures: Vec<Texture>,
}

impl StaticTextures {
    pub fn new(device: &Device, textures: Vec<Texture>) -> Result<Self> {
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

        let descriptor_set_layout =
            descriptors::create_descriptor_set_layout(device, &textures)
                .with_context(trace!(
                    "Unable to create descriptor set layout!"
                ))?;

        let descriptor_pool =
            descriptors::create_descriptor_pool(device, &textures)
                .with_context(trace!(
                    "Unable to create the descriptor pool!"
                ))?;

        let descriptor_set = descriptors::allocate_descriptor_sets(
            device,
            &descriptor_pool,
            &descriptor_set_layout,
        )
        .with_context(trace!("Unable to allocate the descriptor set!"))?;

        descriptors::initialize_descriptor_sets(
            device,
            descriptor_set,
            &sampler,
            &textures,
        );

        Ok(Self {
            descriptor_set,
            _descriptor_pool: descriptor_pool,
            descriptor_set_layout,
            _sampler: sampler,
            _textures: textures,
        })
    }

    /// Gets the active descriptor set layout.
    pub fn descriptor_set_layout(&self) -> &raii::DescriptorSetLayout {
        &self.descriptor_set_layout
    }

    /// Returns a non-owning copy of the descriptor set handle.
    pub fn descriptor_set(&self) -> vk::DescriptorSet {
        self.descriptor_set
    }
}
