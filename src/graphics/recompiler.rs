//! This module provides a tool for automatically recompiling a shader-slang
//! file any time it changes.
//!
//! This works so long as `slangc` is in your PATH. `slangc` is now shipped as
//! part of the Vulkan SDK.

use {
    crate::trace,
    anyhow::{anyhow, bail, Context, Result},
    notify_debouncer_full::{
        new_debouncer,
        notify::{RecursiveMode, Watcher},
        DebounceEventResult,
    },
    std::{
        path::{Path, PathBuf},
        sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError},
        thread::JoinHandle,
        time::Duration,
    },
};

/// Watches a shader source file and recompiles it with `slangc`.
///
/// Note: the recompiler expects to find `slangc` on the system PATH. `slangc`
/// is included in the Vulkan SDK.
pub struct Recompiler {
    compiled_shader_bytes: Vec<u8>,
    compile_thread_join_handle: Option<JoinHandle<()>>,
    shutdown_sender: SyncSender<()>,
    shader_source_receiver: Receiver<Vec<u8>>,
}

impl Recompiler {
    /// Creates a new recompiler that attempts to compile the given shader
    /// source. Returns an error if the initial compilation fails.
    pub fn new(
        shader_source_path: &Path,
        additional_watch_paths: &[PathBuf],
    ) -> Result<Self> {
        let shader_source_path_str = shader_source_path
            .to_str()
            .with_context(trace!("File path isn't a valid utf8 string!"))?
            .to_owned();

        let initial_compiled_shader_bytes =
            try_compile_shader_file(&shader_source_path_str)
                .with_context(trace!("Initial build for shader failed!"))?;

        let (shutdown_sender, shutdown_receiver) = sync_channel::<()>(1);
        let (source_sender, source_receiver) = sync_channel::<Vec<u8>>(1);

        let compile_thread_join_handle = spawn_compiler_thread(
            shader_source_path,
            additional_watch_paths,
            source_sender,
            shutdown_receiver,
        )
        .with_context(trace!("Error while spawning the compiler thread!"))?;

        Ok(Self {
            compiled_shader_bytes: initial_compiled_shader_bytes,
            compile_thread_join_handle: Some(compile_thread_join_handle),
            shutdown_sender,
            shader_source_receiver: source_receiver,
        })
    }

    /// Returns the most up-to-date copy of the shader's compiled SPIR-V bytes.
    pub fn current_shader_bytes(&self) -> &[u8] {
        &self.compiled_shader_bytes
    }

    /// Checks for an updated copy of the compiled source code.
    ///
    /// # Returns
    ///
    /// - true: when there was an updated version of the source available
    /// - false: there was no pending update
    pub fn check_for_update(&mut self) -> Result<bool> {
        match self.shader_source_receiver.try_recv() {
            Ok(new_shader_bytes) => {
                self.compiled_shader_bytes = new_shader_bytes;
                Ok(true)
            }
            Err(TryRecvError::Empty) => Ok(false),
            Err(TryRecvError::Disconnected) => {
                Err(anyhow!(TryRecvError::Disconnected))
                    .with_context(trace!("Compiler thread disconnected!"))
            }
        }
    }
}

impl Drop for Recompiler {
    fn drop(&mut self) {
        self.shutdown_sender
            .send(())
            .expect("Unable to send shutdown signal to the compiler thread!");
        self.compile_thread_join_handle
            .take()
            .unwrap()
            .join()
            .expect("Unable to join compiler thread!");
    }
}

fn spawn_compiler_thread(
    shader_source_path: &Path,
    additional_watch_paths: &[PathBuf],
    source_sender: SyncSender<Vec<u8>>,
    shutdown_receiver: Receiver<()>,
) -> Result<JoinHandle<()>> {
    let additional_watch_paths = additional_watch_paths.to_vec();
    let shader_source_path = shader_source_path.to_owned();
    let shader_source_path_str = shader_source_path.display().to_string();
    let compile_thread_join_handle = std::thread::spawn(move || {
        let mut debouncer =
            new_debouncer(Duration::from_millis(250), None, move |result| {
                handle_debounced_event_result(
                    result,
                    &shader_source_path_str,
                    &source_sender,
                );
            })
            .unwrap();

        debouncer
            .watcher()
            .watch(&shader_source_path, RecursiveMode::NonRecursive)
            .unwrap();

        for additional_path in additional_watch_paths {
            debouncer
                .watcher()
                .watch(&additional_path, RecursiveMode::Recursive)
                .unwrap();
        }

        // block until shutdown
        shutdown_receiver.recv().unwrap();

        debouncer.stop();
    });
    Ok(compile_thread_join_handle)
}

/// Handles a set of debounced file change events to conditionally invoke the
/// compiler.
fn handle_debounced_event_result(
    result: DebounceEventResult,
    shader_source_path_str: &str,
    source_sender: &SyncSender<Vec<u8>>,
) {
    if let Err(err) = result {
        log::error!("Error receiving file change notifications!\n{:#?}", err);
        return;
    }

    match try_compile_shader_file(shader_source_path_str) {
        Ok(shader_src_bytes) => {
            source_sender
                .send(shader_src_bytes)
                .expect("Unable to send updated shader source!");
        }
        Err(e) => {
            log::error!("{}", e);
        }
    }
}

/// Tries to invoke `slangc` on the system PATH to compile a shader file.
///
/// If the shader fails to compile, then a descriptive error message is included
/// in the returned error.
fn try_compile_shader_file(shader_source_path_str: &str) -> Result<Vec<u8>> {
    log::info!("Compiling {}...", shader_source_path_str);
    let output = std::process::Command::new("slangc")
        .args([
            "-matrix-layout-column-major", // compatible with nalgebra
            "-target",
            "spirv",
            "--",
            shader_source_path_str,
        ])
        .output()
        .with_context(trace!("Error executing slangc!"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(trace!("Error when compiling shader!\n\n{}", stderr)());
    }
    log::info!("{} succeeded!", shader_source_path_str);
    Ok(output.stdout)
}
