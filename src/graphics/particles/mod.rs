use {
    crate::{
        graphics::vulkan::{raii, Device, FramesInFlight},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

#[derive(Debug)]
pub struct Particles {
    descriptor_sets: Vec<vk::DescriptorSet>,
    descriptor_set_layout: raii::DescriptorSetLayout,
    descriptor_pool: raii::DescriptorPool,
    // pipeline_layout: raii::PipelineLayout,
    // pipeline: raii::Pipeline,
    device: Arc<Device>,
}

impl Particles {
    pub fn new(
        device: Arc<Device>,
        frames_in_flight: &FramesInFlight,
    ) -> Result<Self> {
        let descriptor_set_layout =
            create_descriptor_set_layout(device.logical_device.clone())
                .with_context(trace!(
                    "Unable to create view descriptor set layout!"
                ))?;

        let descriptor_pool = create_descriptor_pool(
            device.logical_device.clone(),
            frames_in_flight,
        )
        .with_context(trace!("Unable to create view descriptor pool!"))?;

        let descriptor_sets = allocate_descriptor_sets(
            &device,
            &descriptor_pool,
            &descriptor_set_layout,
            frames_in_flight,
        )
        .with_context(trace!("Unable to allocate view descriptor sets!"))?;

        Ok(Self {
            descriptor_sets,
            descriptor_set_layout,
            descriptor_pool,
            device,
        })
    }
}

fn allocate_descriptor_sets(
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

fn create_descriptor_pool(
    logical_device: Arc<raii::Device>,
    frames_in_flight: &FramesInFlight,
) -> Result<raii::DescriptorPool> {
    let pool_sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::UNIFORM_BUFFER,
        descriptor_count: frames_in_flight.frame_count() as u32,
    }];
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

fn create_descriptor_set_layout(
    logical_device: Arc<raii::Device>,
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
        logical_device,
        &vk::DescriptorSetLayoutCreateInfo {
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
            ..Default::default()
        },
    )
}
