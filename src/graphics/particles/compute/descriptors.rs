use {
    crate::{
        graphics::vulkan::{raii, CPUBuffer, Device, UniformBuffer},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

pub fn update_descriptor_sets<T: Copy + Sized, DataT: Copy + Sized>(
    device: &Device,
    descriptor_sets: &[vk::DescriptorSet],
    particles_buffer: &CPUBuffer<T>,
    frame_data: &UniformBuffer<DataT>,
) -> Result<()> {
    let buffer_infos = descriptor_sets
        .iter()
        .enumerate()
        .map(|(index, _)| {
            (
                vk::DescriptorBufferInfo {
                    buffer: particles_buffer.buffer(),
                    offset: 0,
                    range: particles_buffer.size_in_bytes(),
                },
                vk::DescriptorBufferInfo {
                    buffer: frame_data.buffer(),
                    offset: frame_data.offset_for_index(index),
                    range: size_of::<DataT>() as u64,
                },
            )
        })
        .collect::<Vec<_>>();
    let writes = buffer_infos
        .iter()
        .zip(descriptor_sets)
        .flat_map(|((particles_info, frame_data_info), &descriptor_set)| {
            [
                vk::WriteDescriptorSet {
                    dst_set: descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: frame_data_info,
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                },
                vk::WriteDescriptorSet {
                    dst_set: descriptor_set,
                    dst_binding: 1,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: particles_info,
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                },
            ]
        })
        .collect::<Vec<_>>();
    unsafe {
        device.update_descriptor_sets(&writes, &[]);
    }
    Ok(())
}

pub fn allocate_descriptor_sets(
    device: &Device,
    descriptor_pool: &raii::DescriptorPool,
    descriptor_set_layout: &raii::DescriptorSetLayout,
) -> Result<Vec<vk::DescriptorSet>> {
    let layouts = [descriptor_set_layout.raw, descriptor_set_layout.raw];
    let descriptor = unsafe {
        device
            .allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
                descriptor_pool: descriptor_pool.raw,
                descriptor_set_count: 2,
                p_set_layouts: layouts.as_ptr(),
                ..Default::default()
            })
            .with_context(trace!("Unable to allocated descriptor sets!"))?
    };
    Ok(descriptor)
}

pub fn create_descriptor_pool(
    logical_device: Arc<raii::Device>,
) -> Result<raii::DescriptorPool> {
    let pool_sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 2,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 2,
        },
    ];
    raii::DescriptorPool::new(
        logical_device,
        &vk::DescriptorPoolCreateInfo {
            max_sets: 2,
            pool_size_count: pool_sizes.len() as u32,
            p_pool_sizes: pool_sizes.as_ptr(),
            ..Default::default()
        },
    )
}

pub fn create_descriptor_set_layout(
    logical_device: Arc<raii::Device>,
) -> Result<raii::DescriptorSetLayout> {
    let bindings = [
        vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            p_immutable_samplers: std::ptr::null(),
            ..Default::default()
        },
        vk::DescriptorSetLayoutBinding {
            binding: 1,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            p_immutable_samplers: std::ptr::null(),
            ..Default::default()
        },
    ];
    raii::DescriptorSetLayout::new(
        logical_device,
        &vk::DescriptorSetLayoutCreateInfo {
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
            ..Default::default()
        },
    )
}
