use {
    anyhow::{Context, Result},
    ash::vk,
    clap::Parser,
    glfw::{Action, Key, Window, WindowEvent},
    std::sync::Arc,
    sts::{
        app::{app_main, App},
        graphics::vulkan::{
            Device, FrameStatus, FramesInFlight, PresentImageStatus, Swapchain,
        },
        trace,
    },
};

#[derive(Parser, Debug)]
struct Args {}

struct LiveParticles {
    frames_in_flight: FramesInFlight,
    swapchain: Arc<Swapchain>,
    swapchain_needs_rebuild: bool,

    device: Arc<Device>,
}

impl App for LiveParticles {
    type Args = Args;

    fn new(window: &mut glfw::Window, _args: Self::Args) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);

        let device = Device::new(window)
            .with_context(trace!("Unable to create device!"))?;

        let swapchain = {
            let (w, h) = window.get_framebuffer_size();
            Swapchain::new(device.clone(), (w as u32, h as u32), None)
                .with_context(trace!("Unable to create swapchain!"))?
        };

        let frames_in_flight = FramesInFlight::new(device.clone(), 2)
            .with_context(trace!("Unable to create frames_in_flight!"))?;

        Ok(Self {
            frames_in_flight,
            swapchain,
            swapchain_needs_rebuild: false,
            device,
        })
    }

    fn handle_event(
        &mut self,
        window: &mut Window,
        event: WindowEvent,
    ) -> Result<()> {
        if let glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) =
            event
        {
            window.set_should_close(true);
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.swapchain_needs_rebuild {
            self.rebuild_swapchain(window)?;
        }

        let frame = match self.frames_in_flight.start_frame(&self.swapchain)? {
            FrameStatus::FrameStarted(frame) => frame,
            FrameStatus::SwapchainNeedsRebuild => {
                self.swapchain_needs_rebuild = true;
                return Ok(());
            }
        };

        unsafe {
            self.device.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[vk::ImageMemoryBarrier {
                    src_access_mask: vk::AccessFlags::empty(),
                    dst_access_mask: vk::AccessFlags::empty(),
                    old_layout: vk::ImageLayout::UNDEFINED,
                    new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                    src_queue_family_index: self
                        .device
                        .graphics_queue_family_index,
                    dst_queue_family_index: self
                        .device
                        .graphics_queue_family_index,
                    image: self.swapchain.images
                        [frame.swapchain_image_index() as usize],
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    ..Default::default()
                }],
            );
        }

        // TODO: do work here

        if self
            .frames_in_flight
            .present_frame(&self.swapchain, frame)?
            == PresentImageStatus::SwapchainNeedsRebuild
        {
            self.swapchain_needs_rebuild = true;
        }

        Ok(())
    }
}

impl LiveParticles {
    fn rebuild_swapchain(&mut self, window: &mut Window) -> Result<()> {
        self.swapchain_needs_rebuild = false;

        unsafe {
            // wait for all pending work to finish
            self.device.device_wait_idle()?;
        }

        self.swapchain = {
            let (w, h) = window.get_framebuffer_size();
            Swapchain::new(
                self.device.clone(),
                (w as u32, h as u32),
                Some(self.swapchain.raw.raw),
            )?
        };

        // TODO: rebuild render pass and framebuffers

        Ok(())
    }
}

fn main() {
    app_main::<LiveParticles>();
}
