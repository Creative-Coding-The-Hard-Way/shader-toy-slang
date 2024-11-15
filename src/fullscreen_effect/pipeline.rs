use {
    super::FullscreenEffect,
    anyhow::Result,
    ash::vk,
    demo_vk::graphics::vulkan::{raii, spirv_module, VulkanContext},
    std::ffi::CString,
};

impl FullscreenEffect {
    pub(super) fn create_pipeline_layout(
        ctx: &VulkanContext,
        descriptor_set_layouts: &[&raii::DescriptorSetLayout],
    ) -> Result<raii::PipelineLayout> {
        let layouts = descriptor_set_layouts
            .iter()
            .map(|layout| layout.raw)
            .collect::<Vec<vk::DescriptorSetLayout>>();
        raii::PipelineLayout::new(
            ctx.device.clone(),
            &vk::PipelineLayoutCreateInfo {
                set_layout_count: layouts.len() as u32,
                p_set_layouts: layouts.as_ptr(),
                push_constant_range_count: 0,
                p_push_constant_ranges: std::ptr::null(),
                ..Default::default()
            },
        )
    }

    pub(super) fn create_pipeline(
        ctx: &VulkanContext,
        layout: &raii::PipelineLayout,
        render_pass: &raii::RenderPass,
        fragment_shader: &raii::ShaderModule,
    ) -> Result<raii::Pipeline> {
        let entry_point = CString::new("main").unwrap();
        let vertex_shader = spirv_module(
            ctx,
            include_bytes!("./shaders/fullscreen_quad.vert.spv"),
        )?;
        let stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vertex_shader.raw,
                p_name: entry_point.as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: fragment_shader.raw,
                p_name: entry_point.as_ptr(),
                ..Default::default()
            },
        ];
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: 0,
            vertex_attribute_description_count: 0,
            ..Default::default()
        };
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
            ..Default::default()
        };
        let viewport_state = vk::PipelineViewportStateCreateInfo {
            viewport_count: 1,
            scissor_count: 1,
            ..Default::default()
        };
        let raster_state = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::NONE,
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
        let attachment_blend_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        }];
        let blend_state = vk::PipelineColorBlendStateCreateInfo {
            logic_op_enable: vk::FALSE,
            attachment_count: attachment_blend_states.len() as u32,
            p_attachments: attachment_blend_states.as_ptr(),
            ..Default::default()
        };
        let dynamic_states =
            [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state_create_info = vk::PipelineDynamicStateCreateInfo {
            dynamic_state_count: dynamic_states.len() as u32,
            p_dynamic_states: dynamic_states.as_ptr(),
            ..Default::default()
        };
        raii::Pipeline::new_graphics_pipeline(
            ctx.device.clone(),
            &vk::GraphicsPipelineCreateInfo {
                stage_count: stages.len() as u32,
                p_stages: stages.as_ptr(),
                p_vertex_input_state: &vertex_input_state,
                p_input_assembly_state: &input_assembly,
                p_viewport_state: &viewport_state,
                p_rasterization_state: &raster_state,
                p_multisample_state: &multisample_state,
                p_depth_stencil_state: &depth_stencil_state,
                p_color_blend_state: &blend_state,
                p_dynamic_state: &dynamic_state_create_info,
                layout: layout.raw,
                render_pass: render_pass.raw,
                subpass: 0,
                ..Default::default()
            },
        )
    }
}
