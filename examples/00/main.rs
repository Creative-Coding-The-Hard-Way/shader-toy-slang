use {
    anyhow::Result,
    ash::vk,
    glfw::{Action, Key, WindowEvent},
    std::sync::Arc,
    sts::{
        app::{app_main, App},
        graphics::vulkan::{
            AcquireImageStatus, Device, PresentImageStatus, Swapchain,
        },
    },
};

struct Example {
    swapchain: Arc<Swapchain>,
    device: Arc<Device>,
}

impl App for Example {
    fn new(window: &mut glfw::Window) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);

        let device = Device::new(window)?;

        log::debug!("Created device: {:#?}", device);

        let (w, h) = window.get_framebuffer_size();
        let swapchain = Swapchain::new(device.clone(), (w as u32, h as u32))?;

        log::debug!("Created swapchain: {:#?}", swapchain);

        Ok(Self { device, swapchain })
    }

    fn handle_event(
        &mut self,
        window: &mut glfw::Window,
        event: glfw::WindowEvent,
    ) -> Result<()> {
        if let WindowEvent::Key(Key::Escape, _, Action::Release, _) = event {
            window.set_should_close(true);
        }
        Ok(())
    }

    fn update(&mut self, _window: &mut glfw::Window) -> Result<()> {
        let semaphore = {
            let create_info = vk::SemaphoreCreateInfo::default();
            unsafe { self.device.create_semaphore(&create_info, None)? }
        };
        let status = self.swapchain.acquire_image(semaphore)?;
        let index = match status {
            AcquireImageStatus::ImageAcquired(index) => index,
            _ => {
                log::warn!("AOEU");
                return Ok(());
            }
        };

        let result = self.swapchain.present_image(semaphore, index)?;
        if result == PresentImageStatus::SwapchainNeedsRebuild {
            log::warn!("needs rebuild after present");
        }

        unsafe {
            self.device.device_wait_idle()?;
            self.device.destroy_semaphore(semaphore, None);
        }

        Ok(())
    }
}

pub fn main() {
    app_main::<Example>();
}
