use {
    crate::graphics::vulkan::{raii, spirv_module, Swapchain, VulkanContext},
    anyhow::Result,
    ash::vk,
};

/// Creates the pipeline layout based on the required descriptor set layouts.
pub fn create_pipeline_layout(
    ctx: &VulkanContext,
    descriptor_set_layouts: &[&raii::DescriptorSetLayout],
) -> Result<raii::PipelineLayout> {
    let set_layouts = descriptor_set_layouts
        .iter()
        .map(|layout| layout.raw)
        .collect::<Vec<_>>();
    let layout_create_info = vk::PipelineLayoutCreateInfo {
        set_layout_count: set_layouts.len() as u32,
        p_set_layouts: set_layouts.as_ptr(),
        push_constant_range_count: 0,
        p_push_constant_ranges: std::ptr::null(),
        ..Default::default()
    };
    raii::PipelineLayout::new(ctx.device.clone(), &layout_create_info)
}

/// Creates a new pipeline with dynamic viewport and scissor state.
pub fn create_pipeline(
    cxt: &VulkanContext,
    pipeline_layout: &raii::PipelineLayout,
    render_pass: &raii::RenderPass,
    swapchain: &Swapchain,
) -> Result<raii::Pipeline> {
    let main = std::ffi::CString::new("main")?;

    let vertex_shader =
        spirv_module(cxt, include_bytes!("./shaders/sprite.vert.spv"))?;
    let fragment_shader =
        spirv_module(cxt, include_bytes!("./shaders/sprite.frag.spv"))?;
    let stages = [
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::VERTEX,
            module: vertex_shader.raw,
            p_name: main.as_ptr(),
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: fragment_shader.raw,
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
    let viewport = vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: swapchain.extent().width as f32,
        height: swapchain.extent().height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    };
    let scissor = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: swapchain.extent(),
    };
    let viewport_state = vk::PipelineViewportStateCreateInfo {
        viewport_count: 1,
        p_viewports: &viewport,
        scissor_count: 1,
        p_scissors: &scissor,
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
        p_dynamic_state: std::ptr::null(),
        render_pass: render_pass.raw,
        layout: pipeline_layout.raw,
        subpass: 0,
        ..Default::default()
    };
    raii::Pipeline::new_graphics_pipeline(cxt.device.clone(), &create_info)
}
