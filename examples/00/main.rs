use {
    anyhow::Result,
    glfw::{Action, Key, WindowEvent},
    sts::{
        app::{app_main, App},
        graphics::vulkan::Device,
    },
};

struct Example {
    device: Device,
}

impl App for Example {
    fn new(window: &mut glfw::Window) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);

        let device = Device::new(window)?;

        log::debug!("Created device: {:#?}", device);

        Ok(Self { device })
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
