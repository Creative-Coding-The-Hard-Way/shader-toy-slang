//! The GLFW application implementation.
//!
//! This module defines the traits and functions required for managing the
//! lifecycle of a GLFW application with a single Vulkan-enabled window.

mod fullscreen_toggle;
mod logging;
use {
    crate::trace,
    anyhow::{Context, Result},
    clap::Parser,
    glfw::fail_on_errors,
};

pub use self::fullscreen_toggle::FullscreenToggle;

/// Implementations of this trait can be run with app_main to manage a GLFW
/// window.
pub trait App {
    type Args: Sized + Parser;

    /// Creates a new instance of the application.
    /// The application is allowed to modify the window based on its own
    /// requirements. This includes modifying the polling state, fullscreen
    /// status, size, etc...
    fn new(window: &mut glfw::Window, args: Self::Args) -> Result<Self>
    where
        Self: Sized;

    /// Handles a single GLFW event.
    ///
    /// This function is called in a loop to consume any pending events before
    /// every call to update().
    fn handle_event(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] event: glfw::WindowEvent,
    ) -> Result<()> {
        Ok(())
    }

    /// Called in a loop after all pending events have been processed.
    ///
    /// This is a good place for rendering logic. This method blocks event
    /// processing, so it should be kept as responsive as possible.
    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
    ) -> Result<()> {
        Ok(())
    }
}

/// The entrypoint for implementations of the App trait.
///
/// Initializes logging and the GLFW library. Any errors that cause the
/// application to exit are reported with a stacktrace if available.
pub fn app_main<A>()
where
    A: App + 'static,
{
    let exit_result = try_app_main::<A>();
    if let Some(err) = exit_result.err() {
        let result: String = err
            .chain()
            .skip(1)
            .enumerate()
            .map(|(index, err)| format!("  {}| {}\n\n", index, err))
            .to_owned()
            .collect();
        log::error!(
            "{}\n\n{}\n\nCaused by:\n{}\n\nBacktrace:\n{}",
            "Application exited with an error!",
            err,
            result,
            err.backtrace()
        );
    }
}

fn try_app_main<A>() -> Result<()>
where
    A: App + 'static,
{
    logging::setup();

    let args = argfile::expand_args(argfile::parse_fromfile, argfile::PREFIX)
        .with_context(trace!("Error while expanding argfiles!"))?;
    let args = A::Args::parse_from(args);

    let mut glfw = glfw::init(fail_on_errors!())
        .with_context(trace!("Unable to initalize GLFW!"))?;
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::ScaleToMonitor(true));

    let (mut window, events) = glfw
        .create_window(
            640,
            480,
            "Shader Toy - Slang",
            glfw::WindowMode::Windowed,
        )
        .with_context(trace!("Unable to create window!"))?;

    let mut app = A::new(&mut window, args)
        .with_context(trace!("Error while initializing the app!"))?;

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            app.handle_event(&mut window, event)?;
        }

        app.update(&mut window)?;
    }

    Ok(())
}
