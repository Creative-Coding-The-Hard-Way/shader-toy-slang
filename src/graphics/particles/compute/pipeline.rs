use {crate::graphics::vulkan::raii, anyhow::Result, ash::vk, std::sync::Arc};

pub fn create_layout(
    logical_device: Arc<raii::Device>,
    descriptor_set_layout: &raii::DescriptorSetLayout,
) -> Result<raii::PipelineLayout> {
    raii::PipelineLayout::new(
        logical_device,
        &vk::PipelineLayoutCreateInfo {
            set_layout_count: 1,
            p_set_layouts: &descriptor_set_layout.raw,
            push_constant_range_count: 0,
            p_push_constant_ranges: std::ptr::null(),
            ..Default::default()
        },
    )
}

pub fn create_pipeline(
    logical_device: Arc<raii::Device>,
    layout: &raii::PipelineLayout,
    kernel: &raii::ShaderModule,
) -> Result<raii::Pipeline> {
    let main = std::ffi::CString::new("main").unwrap();

    raii::Pipeline::new_compute_pipeline(
        logical_device,
        &vk::ComputePipelineCreateInfo {
            stage: vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::COMPUTE,
                module: kernel.raw,
                p_name: main.as_ptr(),
                p_specialization_info: std::ptr::null(),
                ..Default::default()
            },
            layout: layout.raw,
            ..Default::default()
        },
    )
}
