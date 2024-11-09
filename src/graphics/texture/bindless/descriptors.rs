use {
    crate::graphics::vulkan::{raii, VulkanContext},
    anyhow::Result,
    ash::vk,
};

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
