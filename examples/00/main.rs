use {
    anyhow::{bail, Context, Result},
    glfw::{Action, Key, WindowEvent},
    sts::{
        app::{app_main, App},
        graphics::vulkan::instance::Instance,
    },
};

struct Example {
    instance: Instance,
}

impl App for Example {
    fn new(window: &mut glfw::Window) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_all_polling(true);

        if !window.glfw.vulkan_supported() {
            bail!(sts::trace!("Vulkan not supported on this platform!")());
        }

        let extensions = window
            .glfw
            .get_required_instance_extensions()
            .with_context(sts::trace!(
                "Unable to get required extensions for Vulkan instance!"
            ))?;
        let instance = Instance::new("Example", &extensions)
            .with_context(sts::trace!("Unable to create vulkan instance!"))?;

        log::debug!("Created instance: {:#?}", instance);

        Ok(Self { instance })
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
