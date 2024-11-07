mod compute;
mod view;

use {
    super::vulkan::CPUBuffer,
    crate::graphics::vulkan::{
        raii, Frame, FramesInFlight, Swapchain, VulkanContext,
    },
    anyhow::Result,
    ash::vk,
    bon::bon,
    compute::ParticlesCompute,
    std::sync::Arc,
    view::ParticlesView,
};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct Particle {
    pub pos: [f32; 2],
    pub vel: [f32; 2],
    pub size: f32,
    pub mass: f32,
}

#[derive(Debug)]
pub struct Particles<FrameDataT: Copy + Sized> {
    particles_buffer: CPUBuffer<Particle>,
    view: ParticlesView,
    init: ParticlesCompute<FrameDataT>,
    compute: ParticlesCompute<FrameDataT>,
    init_requested: bool,
    cxt: Arc<VulkanContext>,
}

#[bon]
impl<FrameDataT: Copy + Sized> Particles<FrameDataT> {
    #[builder]
    pub fn new(
        cxt: Arc<VulkanContext>,
        frames_in_flight: &FramesInFlight,
        swapchain: &Swapchain,
        render_pass: &raii::RenderPass,
        kernel: &raii::ShaderModule,
        init: &raii::ShaderModule,
    ) -> Result<Self> {
        let particles_buffer = CPUBuffer::<Particle>::allocate(
            &cxt,
            320,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;

        let view = ParticlesView::builder()
            .cxt(cxt.clone())
            .frames_in_flight(frames_in_flight)
            .swapchain(swapchain)
            .render_pass(render_pass)
            .particles_buffer(&particles_buffer)
            .build()?;

        let compute = ParticlesCompute::builder()
            .cxt(cxt.clone())
            .particles_buffer(&particles_buffer)
            .kernel(kernel)
            .build()?;

        let init = ParticlesCompute::builder()
            .cxt(cxt.clone())
            .particles_buffer(&particles_buffer)
            .kernel(init)
            .build()?;

        Ok(Self {
            particles_buffer,
            view,
            init,
            init_requested: true,
            compute,
            cxt,
        })
    }

    pub fn compute_updated(
        &mut self,
        kernel: &raii::ShaderModule,
        init: &raii::ShaderModule,
        frames_in_flight: &FramesInFlight,
    ) -> Result<()> {
        self.init_requested = true;
        frames_in_flight.wait_for_all_frames_to_complete()?;
        // safe because all frames are stalled
        unsafe {
            self.init.rebuild_kernel(init)?;
            self.compute.rebuild_kernel(kernel)?;
        }
        Ok(())
    }

    pub fn tick(
        &mut self,
        frame: &Frame,
        frame_data: FrameDataT,
    ) -> Result<()> {
        unsafe {
            self.cxt.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::VERTEX_SHADER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier {
                    src_access_mask: vk::AccessFlags::SHADER_READ,
                    dst_access_mask: vk::AccessFlags::SHADER_WRITE,
                    src_queue_family_index: self
                        .cxt
                        .graphics_queue_family_index,
                    dst_queue_family_index: self
                        .cxt
                        .graphics_queue_family_index,
                    buffer: self.particles_buffer.buffer(),
                    offset: 0,
                    size: self.particles_buffer.size_in_bytes(),
                    ..Default::default()
                }],
                &[],
            );
        }
        if self.init_requested {
            self.init.update(frame, frame_data)?;
            self.init_requested = false;
        } else {
            self.compute.update(frame, frame_data)?;
        }
        unsafe {
            self.cxt.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::VERTEX_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier {
                    src_access_mask: vk::AccessFlags::SHADER_WRITE,
                    dst_access_mask: vk::AccessFlags::SHADER_READ,
                    src_queue_family_index: self
                        .cxt
                        .graphics_queue_family_index,
                    dst_queue_family_index: self
                        .cxt
                        .graphics_queue_family_index,
                    buffer: self.particles_buffer.buffer(),
                    offset: 0,
                    size: self.particles_buffer.size_in_bytes(),
                    ..Default::default()
                }],
                &[],
            );
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &Frame) -> Result<()> {
        self.view.draw(frame)?;
        Ok(())
    }

    pub fn swapchain_rebuilt(
        &mut self,
        swapchain: &Swapchain,
        render_pass: &raii::RenderPass,
    ) -> Result<()> {
        self.view.swapchain_rebuilt(swapchain, render_pass)
    }
}
