use {
    super::uniform_buffer::UniformBuffer,
    crate::{
        graphics::vulkan::{raii, Device},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// Allocates the descriptor sets.
pub fn allocate_descriptor_sets(
    device: &Device,
    pool: &raii::DescriptorPool,
    layout: &raii::DescriptorSetLayout,
    count: usize,
) -> Result<Vec<vk::DescriptorSet>> {
    // Allocate N descriptor sets
    let layouts = (0..count)
        .map(|_| layout.raw)
        .collect::<Vec<vk::DescriptorSetLayout>>();
    unsafe {
        let allocate_info = vk::DescriptorSetAllocateInfo {
            descriptor_pool: pool.raw,
            descriptor_set_count: count as u32,
            p_set_layouts: layouts.as_ptr(),
            ..Default::default()
        };
        device
            .allocate_descriptor_sets(&allocate_info)
            .with_context(trace!("Error while allocating descriptor sets!"))
    }
}

/// Updates each descriptor set to refer to the correct location within the
/// uniform buffer.
pub fn initialize_descriptor_sets<FrameDataT>(
    device: &Device,
    descriptor_sets: &[vk::DescriptorSet],
    uniform_buffer: &UniformBuffer<FrameDataT>,
) where
    FrameDataT: Sized + Copy + Default,
{
    // Update each descriptor set to refer to the correct location within the
    // uniform buffer.
    unsafe {
        let buffer_infos = descriptor_sets
            .iter()
            .enumerate()
            .map(|(index, _)| vk::DescriptorBufferInfo {
                buffer: uniform_buffer.buffer.raw,
                offset: uniform_buffer.offset_for_index(index),
                range: std::mem::size_of::<FrameDataT>() as u64,
            })
            .collect::<Vec<_>>();
        let writes = buffer_infos
            .iter()
            .enumerate()
            .map(|(index, buffer_info)| vk::WriteDescriptorSet {
                dst_set: descriptor_sets[index],
                dst_binding: 0,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                p_image_info: std::ptr::null(),
                p_buffer_info: buffer_info,
                p_texel_buffer_view: std::ptr::null(),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        device.update_descriptor_sets(&writes, &[]);
    };
}

/// Creates a new descriptor pool with capacity for `count` uniform buffer
/// descriptors. (one for each frame in flight)
pub fn create_descriptor_pool(
    device: &Device,
    count: usize,
) -> Result<raii::DescriptorPool> {
    let sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::UNIFORM_BUFFER,
        descriptor_count: count as u32,
    }];
    let create_info = vk::DescriptorPoolCreateInfo {
        max_sets: count as u32,
        pool_size_count: sizes.len() as u32,
        p_pool_sizes: sizes.as_ptr(),
        ..Default::default()
    };
    raii::DescriptorPool::new(device.logical_device.clone(), &create_info)
}

/// Creates the descriptor layout. Only one is created because the layout is the
/// same for each frame in flight.
pub fn create_descriptor_set_layout(
    device: &Device,
) -> Result<raii::DescriptorSetLayout> {
    let descriptor_set_bindings = [vk::DescriptorSetLayoutBinding {
        binding: 0,
        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::FRAGMENT
            | vk::ShaderStageFlags::VERTEX,
        ..Default::default()
    }];
    let create_info = vk::DescriptorSetLayoutCreateInfo {
        binding_count: descriptor_set_bindings.len() as u32,
        p_bindings: descriptor_set_bindings.as_ptr(),
        ..Default::default()
    };
    raii::DescriptorSetLayout::new(device.logical_device.clone(), &create_info)
}
