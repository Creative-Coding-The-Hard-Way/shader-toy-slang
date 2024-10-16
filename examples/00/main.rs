use {
    anyhow::Result,
    glfw::{Action, Key, WindowEvent},
    std::sync::Arc,
    sts::{
        app::{app_main, App},
        graphics::vulkan::{Device, Swapchain},
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
        Ok(())
    }
}

pub fn main() {
    app_main::<Example>();
}
