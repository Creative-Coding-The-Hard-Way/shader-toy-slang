use {
    crate::trace,
    anyhow::{Context, Result},
    glfw::WindowMode,
};

/// A helper for toggling fullscreen on a glfw window.
pub struct FullscreenToggle {
    pos: (i32, i32),
    framebuffer_size: (u32, u32),
    is_fullscreen: bool,
}

impl FullscreenToggle {
    /// Create a new fullscreen toggle helper.
    pub fn new(window: &glfw::Window) -> Self {
        let pos = window.get_pos();
        let (w, h) = window.get_framebuffer_size();
        Self {
            pos,
            framebuffer_size: (w as u32, h as u32),
            is_fullscreen: false,
        }
    }

    /// Switch to fullscreen if windowed, switch back to windowed if fullscreen.
    pub fn toggle_fullscreen(
        &mut self,
        window: &mut glfw::Window,
    ) -> Result<()> {
        let mut glfw = window.glfw.clone();
        glfw.with_primary_monitor(|_, monitor_opt| -> Result<()> {
            let monitor = monitor_opt
                .with_context(trace!("no primary monitor detected!"))?;

            match self.is_fullscreen {
                true => {
                    // then switch to "windowed"
                    window.set_decorated(true);
                    window.set_monitor(
                        WindowMode::Windowed,
                        self.pos.0,
                        self.pos.1,
                        self.framebuffer_size.0,
                        self.framebuffer_size.1,
                        None,
                    );
                    self.is_fullscreen = false;
                }
                false => {
                    // switch to "borderless fullscreen"

                    // save the current position and size before switching
                    self.pos = window.get_pos();
                    let (w, h) = window.get_framebuffer_size();
                    self.framebuffer_size = (w as u32, h as u32);

                    // switch to fullscreen
                    let mode = monitor.get_video_mode().with_context(
                        trace!("Unable to get video mode for primary monitor!"),
                    )?;
                    window.set_decorated(false);
                    window.set_monitor(
                        WindowMode::Windowed,
                        0,
                        0,
                        mode.width,
                        mode.height,
                        Some(mode.refresh_rate),
                    );
                    self.is_fullscreen = true;
                }
            }

            Ok(())
        })
    }
}
