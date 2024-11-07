use {
    crate::{
        graphics::vulkan::{raii, Swapchain, VulkanContext},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// Creates a new graphics pipeline that targets the entire swapchain viewport,
/// is compatible with the provided render pass, and uses the provided fragment
/// shader source.
pub fn create_pipeline(
    cxt: &VulkanContext,
    swapchain: &Swapchain,
    render_pass: &raii::RenderPass,
    descriptor_set_layouts: &[&raii::DescriptorSetLayout],
    fragment_shader: &raii::ShaderModule,
) -> Result<(raii::Pipeline, raii::PipelineLayout)> {
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
    let layout =
        raii::PipelineLayout::new(cxt.device.clone(), &layout_create_info)?;

    let main = std::ffi::CString::new("main")?;

    let vertex_shader_words = ash::util::read_spv(&mut std::io::Cursor::new(
        include_bytes!("./shaders/fullscreen_quad.vert.spv"),
    ))?;
    let vertex_module = raii::ShaderModule::new(
        cxt.device.clone(),
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
    let pipeline =
        raii::Pipeline::new_graphics_pipeline(cxt.device.clone(), &create_info)
            .with_context(trace!(
                "Error while creating graphics pipeline for FullscreenQuad!"
            ))?;

    Ok((pipeline, layout))
}
