use {
    anyhow::{Context, Result},
    ash::vk,
    sts::{
        graphics::vulkan::{raii, Device, Swapchain},
        trace,
    },
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(packed)]
pub struct FrameData {
    pub mouse_pos: [f32; 2],
    pub screen_size: [f32; 2],
    pub dt: f32,
    pub time: f32,
}

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

pub fn create_pipeline(
    device: &Device,
    swapchain: &Swapchain,
    render_pass: &raii::RenderPass,
    descriptor_set_layout: &raii::DescriptorSetLayout,
    fragment_shader_source: &[u8],
) -> Result<(raii::Pipeline, raii::PipelineLayout)> {
    let layout_create_info = vk::PipelineLayoutCreateInfo {
        set_layout_count: 1,
        p_set_layouts: &descriptor_set_layout.raw,
        push_constant_range_count: 0,
        p_push_constant_ranges: std::ptr::null(),
        ..Default::default()
    };
    let layout = raii::PipelineLayout::new(
        device.logical_device.clone(),
        &layout_create_info,
    )?;

    let main = std::ffi::CString::new("main")?;

    let fragment_shader_words =
        ash::util::read_spv(&mut std::io::Cursor::new(fragment_shader_source))?;

    let fragment_module = raii::ShaderModule::new(
        device.logical_device.clone(),
        &vk::ShaderModuleCreateInfo {
            code_size: fragment_shader_words.len() * 4,
            p_code: fragment_shader_words.as_ptr(),
            ..Default::default()
        },
    )
    .with_context(trace!("Error while creating fragment shader module!"))?;

    let vertex_shader_words = ash::util::read_spv(&mut std::io::Cursor::new(
        include_bytes!("./shaders/passthrough.vert.spv"),
    ))?;
    let vertex_module = raii::ShaderModule::new(
        device.logical_device.clone(),
        &vk::ShaderModuleCreateInfo {
            code_size: vertex_shader_words.len() * 4,
            p_code: vertex_shader_words.as_ptr(),
            ..Default::default()
        },
    )?;
    let stages = [
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::VERTEX,
            module: vertex_module.raw,
            p_name: main.as_ptr(),
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: fragment_module.raw,
            p_name: main.as_ptr(),
            ..Default::default()
        },
    ];
    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: vk::FALSE,
        ..Default::default()
    };
    let tesselation_state = vk::PipelineTessellationStateCreateInfo::default();
    let viewport = vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: swapchain.extent.width as f32,
        height: swapchain.extent.height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    };
    let scissor = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: swapchain.extent,
    };
    let viewport_state = vk::PipelineViewportStateCreateInfo {
        viewport_count: 1,
        p_viewports: &viewport,
        scissor_count: 1,
        p_scissors: &scissor,
        ..Default::default()
    };
    let rasterization_state = vk::PipelineRasterizationStateCreateInfo {
        depth_clamp_enable: vk::FALSE,
        rasterizer_discard_enable: vk::FALSE,
        polygon_mode: vk::PolygonMode::FILL,
        cull_mode: vk::CullModeFlags::NONE,
        front_face: vk::FrontFace::COUNTER_CLOCKWISE,
        depth_bias_enable: vk::FALSE,
        line_width: 1.0,
        ..Default::default()
    };
    let multisample_state = vk::PipelineMultisampleStateCreateInfo {
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        sample_shading_enable: vk::FALSE,
        ..Default::default()
    };
    let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
        depth_test_enable: vk::FALSE,
        depth_write_enable: vk::FALSE,
        stencil_test_enable: vk::FALSE,
        ..Default::default()
    };
    let attachment = vk::PipelineColorBlendAttachmentState {
        blend_enable: vk::TRUE,
        src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::ONE,
        dst_alpha_blend_factor: vk::BlendFactor::ZERO,
        alpha_blend_op: vk::BlendOp::ADD,
        color_write_mask: vk::ColorComponentFlags::RGBA,
    };
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
        logic_op_enable: vk::FALSE,
        attachment_count: 1,
        p_attachments: &attachment,
        ..Default::default()
    };
    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
        vertex_binding_description_count: 0,
        p_vertex_binding_descriptions: std::ptr::null(),
        vertex_attribute_description_count: 0,
        p_vertex_attribute_descriptions: std::ptr::null(),
        ..Default::default()
    };
    let create_info = vk::GraphicsPipelineCreateInfo {
        stage_count: stages.len() as u32,
        p_stages: stages.as_ptr(),
        p_vertex_input_state: &vertex_input_state,
        p_input_assembly_state: &input_assembly_state,
        p_tessellation_state: &tesselation_state,
        p_viewport_state: &viewport_state,
        p_rasterization_state: &rasterization_state,
        p_multisample_state: &multisample_state,
        p_depth_stencil_state: &depth_stencil_state,
        p_color_blend_state: &color_blend_state,
        render_pass: render_pass.raw,
        layout: layout.raw,
        subpass: 0,
        ..Default::default()
    };
    let pipeline = raii::Pipeline::new_graphics_pipeline(
        device.logical_device.clone(),
        &create_info,
    )?;
    Ok((pipeline, layout))
}
