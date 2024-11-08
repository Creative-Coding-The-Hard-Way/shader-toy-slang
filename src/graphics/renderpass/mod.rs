use {
    super::vulkan::Frame,
    crate::{
        graphics::vulkan::{raii, Swapchain, VulkanContext},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// A utility for managing a Renderpass and Framebuffers for presenting to
/// swapchain images.
///
/// Swapchain images are attached to color attachment 0 and there is no depth
/// buffer.
pub struct SwapchainColorPass {
    extent: vk::Extent2D,
    renderpass: raii::RenderPass,
    framebuffers: Vec<raii::Framebuffer>,
    ctx: Arc<VulkanContext>,
}

impl SwapchainColorPass {
    /// Creates a new Renderpass and Framebuffers that target the Swapchain.
    pub fn new(ctx: Arc<VulkanContext>, swapchain: &Swapchain) -> Result<Self> {
        let renderpass = create_renderpass(ctx.device.clone(), swapchain)
            .with_context(trace!("Error while creating renderpass!"))?;
        let framebuffers = create_framebuffers(&ctx, &renderpass, swapchain)
            .with_context(trace!("Error while creating framebuffers!"))?;
        Ok(Self {
            extent: swapchain.extent(),
            renderpass,
            framebuffers,
            ctx,
        })
    }

    /// Borrow the underlying Vulkan renderpass.
    pub fn renderpass(&self) -> &raii::RenderPass {
        &self.renderpass
    }

    /// Begin the render pass in the current frame targetting the frame's
    /// swapchain image.
    pub fn begin_render_pass(&self, frame: &Frame, clear_color: [f32; 4]) {
        let clear_colors = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: clear_color,
            },
        }];
        unsafe {
            self.ctx.cmd_begin_render_pass(
                frame.command_buffer(),
                &vk::RenderPassBeginInfo {
                    render_pass: self.renderpass.raw,
                    framebuffer: self.framebuffers
                        [frame.swapchain_image_index() as usize]
                        .raw,
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: self.extent,
                    },
                    clear_value_count: clear_colors.len() as u32,
                    p_clear_values: clear_colors.as_ptr(),
                    ..Default::default()
                },
                vk::SubpassContents::INLINE,
            );
        }
    }

    /// Ends the render pass for the current frame.
    pub fn end_render_pass(&self, frame: &Frame) {
        unsafe {
            self.ctx.cmd_end_render_pass(frame.command_buffer());
        }
    }
}

fn create_renderpass(
    device: Arc<raii::Device>,
    swapchain: &Swapchain,
) -> Result<raii::RenderPass> {
    let attachments = [vk::AttachmentDescription {
        format: swapchain.format(),
        samples: vk::SampleCountFlags::TYPE_1,
        load_op: vk::AttachmentLoadOp::CLEAR,
        store_op: vk::AttachmentStoreOp::STORE,
        stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
        initial_layout: vk::ImageLayout::UNDEFINED,
        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        ..Default::default()
    }];
    let color_attachment = [vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    }];
    let subpasses = [vk::SubpassDescription {
        pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
        input_attachment_count: 0,
        p_input_attachments: std::ptr::null(),
        color_attachment_count: color_attachment.len() as u32,
        p_color_attachments: color_attachment.as_ptr(),
        p_resolve_attachments: std::ptr::null(),
        p_depth_stencil_attachment: std::ptr::null(),
        preserve_attachment_count: 0,
        p_preserve_attachments: std::ptr::null(),
        ..Default::default()
    }];
    let dependencies = [
        vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        },
        vk::SubpassDependency {
            src_subpass: 0,
            dst_subpass: vk::SUBPASS_EXTERNAL,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags::empty(),
            dependency_flags: vk::DependencyFlags::empty(),
        },
    ];
    raii::RenderPass::new(
        device,
        &vk::RenderPassCreateInfo {
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: subpasses.len() as u32,
            p_subpasses: subpasses.as_ptr(),
            dependency_count: dependencies.len() as u32,
            p_dependencies: dependencies.as_ptr(),
            ..Default::default()
        },
    )
}

/// Creates one framebuffer per swapchain image view.
///
/// Framebuffers must be replaced when the swapchain is rebuilt.
fn create_framebuffers(
    cxt: &VulkanContext,
    render_pass: &raii::RenderPass,
    swapchain: &Swapchain,
) -> Result<Vec<raii::Framebuffer>> {
    let mut framebuffers = vec![];
    let vk::Extent2D { width, height } = swapchain.extent();
    for image_view in swapchain.image_views() {
        let create_info = vk::FramebufferCreateInfo {
            render_pass: render_pass.raw,
            attachment_count: 1,
            p_attachments: &image_view.raw,
            width,
            height,
            layers: 1,
            ..Default::default()
        };
        framebuffers
            .push(raii::Framebuffer::new(cxt.device.clone(), &create_info)?);
    }
    Ok(framebuffers)
}
