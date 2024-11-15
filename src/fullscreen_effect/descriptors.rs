use {
    super::{FrameData, FullscreenEffect},
    anyhow::Result,
    ash::vk,
    demo_vk::graphics::vulkan::{raii, UniformBuffer, VulkanContext},
};

impl FullscreenEffect {
    pub(super) fn update_descriptor_sets(
        ctx: &VulkanContext,
        descriptor_sets: &[vk::DescriptorSet],
        uniform_buffer: &UniformBuffer<FrameData>,
    ) {
        let buffer_infos = descriptor_sets
            .iter()
            .enumerate()
            .map(|(index, _)| vk::DescriptorBufferInfo {
                buffer: uniform_buffer.buffer(),
                offset: uniform_buffer.offset_for_index(index),
                range: size_of::<FrameData>() as u64,
            })
            .collect::<Vec<vk::DescriptorBufferInfo>>();
        let writes = descriptor_sets
            .iter()
            .zip(&buffer_infos)
            .map(|(&dst_set, buffer_info)| vk::WriteDescriptorSet {
                dst_set,
                dst_binding: 0,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                p_image_info: std::ptr::null(),
                p_buffer_info: buffer_info,
                p_texel_buffer_view: std::ptr::null(),
                ..Default::default()
            })
            .collect::<Vec<vk::WriteDescriptorSet>>();
        unsafe {
            ctx.update_descriptor_sets(&writes, &[]);
        }
    }

    pub(super) fn allocate_descriptor_sets(
        ctx: &VulkanContext,
        pool: &raii::DescriptorPool,
        layout: &raii::DescriptorSetLayout,
        count: u32,
    ) -> Result<Vec<vk::DescriptorSet>> {
        let layouts = vec![layout.raw; count as usize];
        let descriptor_sets = unsafe {
            ctx.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
                descriptor_pool: pool.raw,
                descriptor_set_count: count,
                p_set_layouts: layouts.as_ptr(),
                ..Default::default()
            })?
        };
        Ok(descriptor_sets)
    }

    pub(super) fn create_descriptor_pool(
        ctx: &VulkanContext,
        count: u32,
    ) -> Result<raii::DescriptorPool> {
        let sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: count,
        }];
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

    pub(super) fn create_descriptor_set_laout(
        ctx: &VulkanContext,
    ) -> Result<raii::DescriptorSetLayout> {
        let bindings = [vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::FRAGMENT,
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
}
