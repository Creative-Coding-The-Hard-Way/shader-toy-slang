mod compute;
mod view;

use {
    super::vulkan::CPUBuffer,
    crate::graphics::vulkan::{raii, Device, Frame, FramesInFlight, Swapchain},
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
pub struct Particles {
    particles_buffer: CPUBuffer<Particle>,
    view: ParticlesView,
    compute: ParticlesCompute,
    device: Arc<Device>,
}

#[bon]
impl Particles {
    #[builder]
    pub fn new(
        device: Arc<Device>,
        frames_in_flight: &FramesInFlight,
        swapchain: &Swapchain,
        render_pass: &raii::RenderPass,
        kernel_bytes: &[u8],
    ) -> Result<Self> {
        let mut particles_buffer = CPUBuffer::<Particle>::allocate(
            &device,
            1,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        unsafe {
            particles_buffer.write_data(
                0,
                &[Particle {
                    pos: [5.0, 5.0],
                    vel: [0.1, 0.1],
                    size: 1.0,
                    mass: 1.0,
                }],
            )?;
        }

        let view = ParticlesView::builder()
            .device(device.clone())
            .frames_in_flight(frames_in_flight)
            .swapchain(swapchain)
            .render_pass(render_pass)
            .particles_buffer(&particles_buffer)
            .build()?;

        let compute = ParticlesCompute::builder()
            .device(device.clone())
            .particles_buffer(&particles_buffer)
            .kernel_bytes(kernel_bytes)
            .build()?;

        Ok(Self {
            particles_buffer,
            view,
            compute,
            device,
        })
    }

    pub fn compute_updated(
        &mut self,
        kernel_bytes: &[u8],
        frames_in_flight: &FramesInFlight,
    ) -> Result<()> {
        frames_in_flight.wait_for_all_frames_to_complete()?;
        // safe because all frames are stalled
        unsafe { self.compute.rebuild_kernel(kernel_bytes) }
    }

    pub fn tick(&mut self, frame: &Frame) -> Result<()> {
        unsafe {
            self.device.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::VERTEX_SHADER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier {
                    src_access_mask: vk::AccessFlags::SHADER_READ,
                    dst_access_mask: vk::AccessFlags::SHADER_WRITE,
                    src_queue_family_index: self
                        .device
                        .graphics_queue_family_index,
                    dst_queue_family_index: self
                        .device
                        .graphics_queue_family_index,
                    buffer: self.particles_buffer.buffer(),
                    offset: 0,
                    size: self.particles_buffer.size_in_bytes(),
                    ..Default::default()
                }],
                &[],
            );
        }
        self.compute.update(frame)?;
        unsafe {
            self.device.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::VERTEX_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier {
                    src_access_mask: vk::AccessFlags::SHADER_WRITE,
                    dst_access_mask: vk::AccessFlags::SHADER_READ,
                    src_queue_family_index: self
                        .device
                        .graphics_queue_family_index,
                    dst_queue_family_index: self
                        .device
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
