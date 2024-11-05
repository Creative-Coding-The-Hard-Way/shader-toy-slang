use {
    crate::{
        graphics::{
            particles::Particle,
            vulkan::{raii, CPUBuffer, Device, FramesInFlight, UniformBuffer},
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

pub fn update_descriptor_sets<T: Copy + Sized>(
    device: &Device,
    descriptor_sets: &[vk::DescriptorSet],
    uniform_buffer: &UniformBuffer<T>,
    particles_buffer: &CPUBuffer<Particle>,
) -> Result<()> {
    let particles_buffer_info = descriptor_sets
        .iter()
        .map(|_| vk::DescriptorBufferInfo {
            buffer: particles_buffer.buffer(),
            offset: 0,
            range: particles_buffer.size_in_bytes(),
        })
        .collect::<Vec<vk::DescriptorBufferInfo>>();
    let buffer_infos = descriptor_sets
        .iter()
        .enumerate()
        .map(|(index, _)| vk::DescriptorBufferInfo {
            buffer: uniform_buffer.buffer(),
            offset: uniform_buffer.offset_for_index(index),
            range: size_of::<T>() as u64,
        })
        .collect::<Vec<vk::DescriptorBufferInfo>>();
    let writes = buffer_infos
        .iter()
        .zip(particles_buffer_info.iter())
        .zip(descriptor_sets)
        .flat_map(|((buffer_info, particles_buffer_info), &descriptor_set)| {
            [
                vk::WriteDescriptorSet {
                    dst_set: descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: buffer_info,
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
                    p_buffer_info: particles_buffer_info,
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                },
            ]
        })
        .collect::<Vec<vk::WriteDescriptorSet>>();
    unsafe {
        device.update_descriptor_sets(&writes, &[]);
    }
    Ok(())
}

pub fn allocate_descriptor_sets(
    device: &Device,
    descriptor_pool: &raii::DescriptorPool,
    descriptor_set_layout: &raii::DescriptorSetLayout,
    frames_in_flight: &FramesInFlight,
) -> Result<Vec<vk::DescriptorSet>> {
    let layouts = (0..frames_in_flight.frame_count())
        .map(|_| descriptor_set_layout.raw)
        .collect::<Vec<vk::DescriptorSetLayout>>();
    unsafe {
        device
            .allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
                descriptor_pool: descriptor_pool.raw,
                descriptor_set_count: frames_in_flight.frame_count() as u32,
                p_set_layouts: layouts.as_ptr(),
                ..Default::default()
            })
            .with_context(trace!("Unable to allocated descriptor sets!"))
    }
}

pub fn create_descriptor_pool(
    logical_device: Arc<raii::Device>,
    frames_in_flight: &FramesInFlight,
) -> Result<raii::DescriptorPool> {
    let pool_sizes = [
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: frames_in_flight.frame_count() as u32,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: frames_in_flight.frame_count() as u32,
        },
    ];
    raii::DescriptorPool::new(
        logical_device,
        &vk::DescriptorPoolCreateInfo {
            max_sets: frames_in_flight.frame_count() as u32,
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
            stage_flags: vk::ShaderStageFlags::VERTEX,
            p_immutable_samplers: std::ptr::null(),
            ..Default::default()
        },
        vk::DescriptorSetLayoutBinding {
            binding: 1,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::VERTEX,
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
