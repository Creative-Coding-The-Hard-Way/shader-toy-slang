use {
    crate::graphics::vulkan::{raii, DescriptorBumpAllocator, VulkanContext},
    anyhow::Result,
    ash::vk,
    std::sync::Arc,
};

pub fn create_descriptor_allocator(
    ctx: Arc<VulkanContext>,
) -> Result<DescriptorBumpAllocator> {
    DescriptorBumpAllocator::new(
        ctx,
        20,
        [
            (vk::DescriptorType::UNIFORM_BUFFER, 1),
            (vk::DescriptorType::STORAGE_BUFFER, 1),
        ],
    )
}

pub fn create_layer_descriptor_set_layout(
    ctx: &VulkanContext,
) -> Result<raii::DescriptorSetLayout> {
    let bindings = [vk::DescriptorSetLayoutBinding {
        binding: 0,
        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::VERTEX,
        p_immutable_samplers: std::ptr::null(),
        ..Default::default()
    }];
    raii::DescriptorSetLayout::new(
        ctx.device.clone(),
        &vk::DescriptorSetLayoutCreateInfo {
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
            ..Default::default()
        },
    )
}

pub fn create_batch_descriptor_set_layout(
    ctx: &VulkanContext,
) -> Result<raii::DescriptorSetLayout> {
    let bindings = [vk::DescriptorSetLayoutBinding {
        binding: 0,
        descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::VERTEX,
        p_immutable_samplers: std::ptr::null(),
        ..Default::default()
    }];
    raii::DescriptorSetLayout::new(
        ctx.device.clone(),
        &vk::DescriptorSetLayoutCreateInfo {
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
            ..Default::default()
        },
    )
}
