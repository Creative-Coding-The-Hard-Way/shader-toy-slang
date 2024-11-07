use {
    crate::graphics::vulkan::{raii, spirv_words, Swapchain},
    anyhow::Result,
    ash::vk,
    std::sync::Arc,
};

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
    swapchain: &Swapchain,
    render_pass: &raii::RenderPass,
    pipeline_layout: &raii::PipelineLayout,
) -> Result<raii::Pipeline> {
    let vertex_source =
        spirv_words(include_bytes!("./shaders/particle.vert.spv"))?;
    let fragment_source =
        spirv_words(include_bytes!("./shaders/particle.frag.spv"))?;

    let vertex_module = raii::ShaderModule::new(
        logical_device.clone(),
        &vk::ShaderModuleCreateInfo {
            code_size: vertex_source.len() * 4,
            p_code: vertex_source.as_ptr(),
            ..Default::default()
        },
    )?;
    let fragment_module = raii::ShaderModule::new(
        logical_device.clone(),
        &vk::ShaderModuleCreateInfo {
            code_size: fragment_source.len() * 4,
            p_code: fragment_source.as_ptr(),
            ..Default::default()
        },
    )?;
    let main = std::ffi::CString::new("main")?;
    let stages = [
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::VERTEX,
            module: vertex_module.raw,
            p_name: main.as_ptr(),
            p_specialization_info: std::ptr::null(),
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: fragment_module.raw,
            p_name: main.as_ptr(),
            p_specialization_info: std::ptr::null(),
            ..Default::default()
        },
    ];

    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
        vertex_binding_description_count: 0,
        p_vertex_binding_descriptions: std::ptr::null(),
        vertex_attribute_description_count: 0,
        p_vertex_attribute_descriptions: std::ptr::null(),
        ..Default::default()
    };

    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: vk::FALSE,
        ..Default::default()
    };

    let viewports = [vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: swapchain.extent().width as f32,
        height: swapchain.extent().height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    }];
    let scissors = [vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: swapchain.extent(),
    }];
    let viewport_state = vk::PipelineViewportStateCreateInfo {
        viewport_count: viewports.len() as u32,
        p_viewports: viewports.as_ptr(),
        scissor_count: scissors.len() as u32,
        p_scissors: scissors.as_ptr(),
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
        depth_bounds_test_enable: vk::FALSE,
        stencil_test_enable: vk::FALSE,
        ..Default::default()
    };

    let blend_attachments = [vk::PipelineColorBlendAttachmentState {
        blend_enable: vk::TRUE,
        src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::ONE,
        dst_alpha_blend_factor: vk::BlendFactor::ZERO,
        alpha_blend_op: vk::BlendOp::ADD,
        color_write_mask: vk::ColorComponentFlags::RGBA,
    }];
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
        logic_op_enable: vk::FALSE,
        attachment_count: blend_attachments.len() as u32,
        p_attachments: blend_attachments.as_ptr(),
        ..Default::default()
    };

    let dynamic_state = vk::PipelineDynamicStateCreateInfo {
        dynamic_state_count: 0,
        p_dynamic_states: std::ptr::null(),
        ..Default::default()
    };

    let create_info = vk::GraphicsPipelineCreateInfo {
        stage_count: stages.len() as u32,
        p_stages: stages.as_ptr(),
        p_vertex_input_state: &vertex_input_state,
        p_input_assembly_state: &input_assembly_state,
        p_tessellation_state: std::ptr::null(),
        p_viewport_state: &viewport_state,
        p_rasterization_state: &rasterization_state,
        p_multisample_state: &multisample_state,
        p_depth_stencil_state: &depth_stencil_state,
        p_color_blend_state: &color_blend_state,
        p_dynamic_state: &dynamic_state,
        layout: pipeline_layout.raw,
        render_pass: render_pass.raw,
        subpass: 0,
        ..Default::default()
    };
    raii::Pipeline::new_graphics_pipeline(logical_device, &create_info)
}
