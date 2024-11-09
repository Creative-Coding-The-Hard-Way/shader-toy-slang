use {
    crate::graphics::vulkan::{raii, VulkanContext},
    anyhow::Result,
    ash::vk,
};

pub fn allocate_descriptor_sets(
    ctx: &VulkanContext,
    descriptor_pool: &raii::DescriptorPool,
    descriptor_set_layout: &raii::DescriptorSetLayout,
    count: u32,
) -> Result<Vec<vk::DescriptorSet>> {
    let layouts = vec![descriptor_set_layout.raw; count as usize];
    let descriptor_sets = unsafe {
        ctx.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
            descriptor_pool: descriptor_pool.raw,
            descriptor_set_count: layouts.len() as u32,
            p_set_layouts: layouts.as_ptr(),
            ..Default::default()
        })?
    };
    Ok(descriptor_sets)
}

pub fn create_descriptor_layout(
    ctx: &VulkanContext,
    max_textures: u32,
    samplers: &[raii::Sampler],
) -> Result<raii::DescriptorSetLayout> {
    let samplers = samplers
        .iter()
        .map(|sampler| sampler.raw)
        .collect::<Vec<vk::Sampler>>();
    let bindings = [
        vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::SAMPLER,
            descriptor_count: 3,
            stage_flags: vk::ShaderStageFlags::ALL,
            p_immutable_samplers: samplers.as_ptr(),
            ..Default::default()
        },
        vk::DescriptorSetLayoutBinding {
            binding: 1,
            descriptor_type: vk::DescriptorType::SAMPLED_IMAGE,
            descriptor_count: max_textures,
            stage_flags: vk::ShaderStageFlags::ALL,
            p_immutable_samplers: std::ptr::null(),
            ..Default::default()
        },
    ];
    raii::DescriptorSetLayout::new(
        ctx.device.clone(),
        &vk::DescriptorSetLayoutCreateInfo {
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
            ..Default::default()
        },
    )
}

pub fn create_descriptor_pool(
    ctx: &VulkanContext,
    count: u32,
    max_textures: u32,
) -> Result<raii::DescriptorPool> {
    let sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::SAMPLER,
            descriptor_count: 3 * count,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::SAMPLED_IMAGE,
            descriptor_count: 3 * max_textures,
        },
    ];
    raii::DescriptorPool::new(
        ctx.device.clone(),
        &vk::DescriptorPoolCreateInfo {
            max_sets: count,
            pool_size_count: sizes.len() as u32,
            p_pool_sizes: sizes.as_ptr(),
            ..Default::default()
        },
    )
}
