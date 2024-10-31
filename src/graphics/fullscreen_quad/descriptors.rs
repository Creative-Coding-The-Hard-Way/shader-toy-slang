use {
    super::uniform_buffer::UniformBuffer,
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
    texture: &Option<Texture>,
    sampler: &raii::Sampler,
) where
    FrameDataT: Sized + Copy + Default,
{
    let texture = texture.as_ref();

    let image_info = vk::DescriptorImageInfo {
        sampler: sampler.raw,
        image_view: texture
            .map_or(vk::ImageView::null(), |texture| texture.view()),
        image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    };
    let sampler_info = vk::DescriptorImageInfo {
        sampler: sampler.raw,
        ..Default::default()
    };
    log::info!("{:#?}", image_info);

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
            .flat_map(|(index, buffer_info)| {
                if texture.is_some() {
                    vec![
                        vk::WriteDescriptorSet {
                            dst_set: descriptor_sets[index],
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
                            dst_set: descriptor_sets[index],
                            dst_binding: 1,
                            dst_array_element: 0,
                            descriptor_count: 1,
                            descriptor_type: vk::DescriptorType::SAMPLER,
                            p_image_info: &sampler_info,
                            p_buffer_info: std::ptr::null(),
                            p_texel_buffer_view: std::ptr::null(),
                            ..Default::default()
                        },
                        vk::WriteDescriptorSet {
                            dst_set: descriptor_sets[index],
                            dst_binding: 2,
                            dst_array_element: 0,
                            descriptor_count: 1,
                            descriptor_type: vk::DescriptorType::SAMPLED_IMAGE,
                            p_image_info: &image_info,
                            p_buffer_info: std::ptr::null(),
                            p_texel_buffer_view: std::ptr::null(),
                            ..Default::default()
                        },
                    ]
                } else {
                    vec![vk::WriteDescriptorSet {
                        dst_set: descriptor_sets[index],
                        dst_binding: 0,
                        dst_array_element: 0,
                        descriptor_count: 1,
                        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                        p_image_info: std::ptr::null(),
                        p_buffer_info: buffer_info,
                        p_texel_buffer_view: std::ptr::null(),
                        ..Default::default()
                    }]
                }
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
    texture: &Option<Texture>,
) -> Result<raii::DescriptorPool> {
    let sizes = if texture.is_some() {
        vec![
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: count as u32,
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::SAMPLER,
                descriptor_count: count as u32,
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::SAMPLED_IMAGE,
                descriptor_count: count as u32,
            },
        ]
    } else {
        vec![vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: count as u32,
        }]
    };
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
    texture: &Option<Texture>,
) -> Result<raii::DescriptorSetLayout> {
    let descriptor_set_bindings = if texture.is_some() {
        vec![
            vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::FRAGMENT
                    | vk::ShaderStageFlags::VERTEX,
                ..Default::default()
            },
            // texture
            vk::DescriptorSetLayoutBinding {
                binding: 1,
                descriptor_type: vk::DescriptorType::SAMPLER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::FRAGMENT,
                ..Default::default()
            },
            vk::DescriptorSetLayoutBinding {
                binding: 2,
                descriptor_type: vk::DescriptorType::SAMPLED_IMAGE,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::FRAGMENT,
                ..Default::default()
            },
        ]
    } else {
        vec![vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::FRAGMENT
                | vk::ShaderStageFlags::VERTEX,
            ..Default::default()
        }]
    };

    let create_info = vk::DescriptorSetLayoutCreateInfo {
        binding_count: descriptor_set_bindings.len() as u32,
        p_bindings: descriptor_set_bindings.as_ptr(),
        ..Default::default()
    };
    raii::DescriptorSetLayout::new(device.logical_device.clone(), &create_info)
}
