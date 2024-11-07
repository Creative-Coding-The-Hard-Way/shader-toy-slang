use {
    crate::graphics::{
        vulkan::{raii, VulkanContext},
        Texture,
    },
    anyhow::Result,
    ash::vk,
};

/// Allocates the descriptor sets.
pub fn allocate_descriptor_sets(
    cxt: &VulkanContext,
    pool: &raii::DescriptorPool,
    layout: &raii::DescriptorSetLayout,
) -> Result<vk::DescriptorSet> {
    let descriptor_set = unsafe {
        let allocate_info = vk::DescriptorSetAllocateInfo {
            descriptor_pool: pool.raw,
            descriptor_set_count: 1,
            p_set_layouts: &layout.raw,
            ..Default::default()
        };
        cxt.allocate_descriptor_sets(&allocate_info)?[0]
    };
    Ok(descriptor_set)
}

/// Writes textures and samplers to their respective descriptor set bindings.
pub fn initialize_descriptor_sets(
    cxt: &VulkanContext,
    descriptor_set: vk::DescriptorSet,
    sampler: &raii::Sampler,
    textures: &[Texture],
) {
    let sampler_info = vk::DescriptorImageInfo {
        sampler: sampler.raw,
        ..Default::default()
    };
    let mut writes = vec![vk::WriteDescriptorSet {
        dst_set: descriptor_set,
        dst_binding: 0,
        dst_array_element: 0,
        descriptor_count: 1,
        descriptor_type: vk::DescriptorType::SAMPLER,
        p_image_info: &sampler_info,
        p_buffer_info: std::ptr::null(),
        p_texel_buffer_view: std::ptr::null(),
        ..Default::default()
    }];

    let image_infos = textures
        .iter()
        .map(|texture| vk::DescriptorImageInfo {
            sampler: vk::Sampler::null(),
            image_view: texture.view(),
            image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        })
        .collect::<Vec<_>>();

    writes.extend(image_infos.iter().enumerate().map(|(index, info)| {
        vk::WriteDescriptorSet {
            dst_set: descriptor_set,
            dst_binding: (1 + index) as u32,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::SAMPLED_IMAGE,
            p_image_info: info,
            p_buffer_info: std::ptr::null(),
            p_texel_buffer_view: std::ptr::null(),
            ..Default::default()
        }
    }));

    unsafe {
        cxt.update_descriptor_sets(&writes, &[]);
    };
}

/// Creates a new descriptor pool with capacity for all textures and samplers.
pub fn create_descriptor_pool(
    cxt: &VulkanContext,
    textures: &[Texture],
) -> Result<raii::DescriptorPool> {
    let mut sizes = vec![vk::DescriptorPoolSize {
        ty: vk::DescriptorType::SAMPLER,
        descriptor_count: 1,
    }];

    if !textures.is_empty() {
        sizes.push(vk::DescriptorPoolSize {
            ty: vk::DescriptorType::SAMPLED_IMAGE,
            descriptor_count: textures.len() as u32,
        });
    }

    let create_info = vk::DescriptorPoolCreateInfo {
        max_sets: 1,
        pool_size_count: sizes.len() as u32,
        p_pool_sizes: sizes.as_ptr(),
        ..Default::default()
    };
    raii::DescriptorPool::new(cxt.device.clone(), &create_info)
}

/// Creates the descriptor layout.
pub fn create_descriptor_set_layout(
    cxt: &VulkanContext,
    textures: &[Texture],
) -> Result<raii::DescriptorSetLayout> {
    let mut descriptor_set_bindings = vec![vk::DescriptorSetLayoutBinding {
        binding: 0,
        descriptor_type: vk::DescriptorType::SAMPLER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::FRAGMENT,
        ..Default::default()
    }];

    for (i, _) in textures.iter().enumerate() {
        descriptor_set_bindings.push(vk::DescriptorSetLayoutBinding {
            binding: (1 + i) as u32,
            descriptor_type: vk::DescriptorType::SAMPLED_IMAGE,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        })
    }

    let create_info = vk::DescriptorSetLayoutCreateInfo {
        binding_count: descriptor_set_bindings.len() as u32,
        p_bindings: descriptor_set_bindings.as_ptr(),
        ..Default::default()
    };
    raii::DescriptorSetLayout::new(cxt.device.clone(), &create_info)
}
