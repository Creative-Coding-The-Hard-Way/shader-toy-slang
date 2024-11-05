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
    kernel_bytes: &[u8],
) -> Result<raii::Pipeline> {
    let module = {
        let shader_words =
            ash::util::read_spv(&mut std::io::Cursor::new(kernel_bytes))?;
        raii::ShaderModule::new(
            logical_device.clone(),
            &vk::ShaderModuleCreateInfo {
                code_size: shader_words.len() * 4,
                p_code: shader_words.as_ptr(),
                ..Default::default()
            },
        )?
    };

    let main = std::ffi::CString::new("main").unwrap();

    raii::Pipeline::new_compute_pipeline(
        logical_device,
        &vk::ComputePipelineCreateInfo {
            stage: vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::COMPUTE,
                module: module.raw,
                p_name: main.as_ptr(),
                p_specialization_info: std::ptr::null(),
                ..Default::default()
            },
            layout: layout.raw,
            ..Default::default()
        },
    )
}
