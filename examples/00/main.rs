mod buffer;
mod pipeline;

use {
    anyhow::{Context, Result},
    ash::vk,
    buffer::UniformBuffer,
    clap::Parser,
    glfw::{Action, Key, WindowEvent, WindowMode},
    pipeline::{
        create_descriptor_pool, create_descriptor_set_layout, FrameData,
    },
    std::{
        path::PathBuf,
        sync::Arc,
        time::{Duration, Instant},
    },
    sts::{
        app::{app_main, App, FullscreenToggle},
        graphics::{
            vulkan::{
                raii, Device, FrameStatus, FramesInFlight, PresentImageStatus,
                Swapchain,
            },
            Recompiler,
        },
        trace,
    },
};

#[derive(Parser, Debug, Eq, PartialEq)]
#[command(version, about, long_about=None)]
struct Args {
    /// The path to the shader to watch.
    pub fragment_shader_path: PathBuf,
}

struct Example {
    device: Arc<Device>,
    start_time: Instant,
    last_frame: Instant,

    swapchain: Arc<Swapchain>,
    swapchain_needs_rebuild: bool,
    frames_in_flight: FramesInFlight,

    render_pass: raii::RenderPass,
    framebuffers: Vec<raii::Framebuffer>,

    pipeline: raii::Pipeline,
    pipeline_layout: raii::PipelineLayout,
    descriptor_set_layout: raii::DescriptorSetLayout,
    _descriptor_pool: raii::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
    uniform_buffer: UniformBuffer,

    fragment_shader_compiler: Recompiler,
    fullscreen_toggle: FullscreenToggle,
}

impl Example {
    fn rebuild_swapchain(&mut self, window: &mut glfw::Window) -> Result<()> {
        unsafe {
            // wait for all pending work to finish
            self.device.device_wait_idle()?;
        }

        self.framebuffers.clear();

        let (w, h) = window.get_framebuffer_size();
        self.swapchain = Swapchain::new(
            self.device.clone(),
            (w as u32, h as u32),
            Some(self.swapchain.raw.raw),
        )?;

        self.render_pass = create_renderpass(&self.device, &self.swapchain)?;
        self.framebuffers = create_framebuffers(
            &self.device,
            &self.render_pass,
            &self.swapchain,
        )?;

        log::info!("{:#?}", self.swapchain);
        self.fragment_shader_compiler.check_for_update()?;
        let (pipeline, pipeline_layout) = pipeline::create_pipeline(
            &self.device,
            &self.swapchain,
            &self.render_pass,
            &self.descriptor_set_layout,
            self.fragment_shader_compiler.current_shader_bytes(),
        )?;
        self.pipeline = pipeline;
        self.pipeline_layout = pipeline_layout;

        Ok(())
    }
}

impl App for Example {
    fn new(window: &mut glfw::Window) -> Result<Self>
    where
        Self: Sized,
    {
        let cli_args = Args::try_parse()
            .with_context(trace!("Unable to parse cli args!"))?;

        window.set_all_polling(true);

        let device = Device::new(window)?;
        log::debug!("Created device: {:#?}", device);

        let fragment_shader_compiler =
            Recompiler::new(&cli_args.fragment_shader_path).with_context(
                trace!("Unable to start the fragment shader compiler!"),
            )?;

        let (w, h) = window.get_framebuffer_size();
        let swapchain =
            Swapchain::new(device.clone(), (w as u32, h as u32), None)?;
        log::debug!("Created swapchain: {:#?}", swapchain);

        let frames_in_flight = FramesInFlight::new(device.clone(), 3)?;

        let render_pass = create_renderpass(&device, &swapchain)?;
        let descriptor_set_layout = create_descriptor_set_layout(&device)?;
        let framebuffers =
            create_framebuffers(&device, &render_pass, &swapchain)?;
        let (pipeline, pipeline_layout) = pipeline::create_pipeline(
            &device,
            &swapchain,
            &render_pass,
            &descriptor_set_layout,
            fragment_shader_compiler.current_shader_bytes(),
        )?;

        let descriptor_pool =
            create_descriptor_pool(&device, frames_in_flight.frame_count())?;
        let layouts = (0..frames_in_flight.frame_count())
            .map(|_| descriptor_set_layout.raw)
            .collect::<Vec<vk::DescriptorSetLayout>>();
        let descriptor_sets = unsafe {
            let allocate_info = vk::DescriptorSetAllocateInfo {
                descriptor_pool: descriptor_pool.raw,
                descriptor_set_count: frames_in_flight.frame_count() as u32,
                p_set_layouts: layouts.as_ptr(),
                ..Default::default()
            };
            device.allocate_descriptor_sets(&allocate_info)?
        };
        let mut uniform_buffer = UniformBuffer::allocate::<FrameData>(
            &device,
            frames_in_flight.frame_count(),
        )?;
        for index in 0..frames_in_flight.frame_count() {
            unsafe {
                uniform_buffer.write_indexed(
                    index,
                    FrameData {
                        mouse_pos: [0.0, 0.0],
                        screen_size: [1.0, 1.0],
                        time: 0.0,
                        dt: 0.0,
                    },
                )?;
            }
        }
        unsafe {
            let buffer_infos = (0..frames_in_flight.frame_count())
                .map(|index| vk::DescriptorBufferInfo {
                    buffer: uniform_buffer.buffer.raw,
                    offset: uniform_buffer.offset_for_index(index),
                    range: std::mem::size_of::<FrameData>() as u64,
                })
                .collect::<Vec<_>>();
            let writes = buffer_infos
                .iter()
                .enumerate()
                .map(|(index, buffer_info)| vk::WriteDescriptorSet {
                    dst_set: descriptor_sets[index],
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: buffer_info,
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                })
                .collect::<Vec<_>>();
            device.update_descriptor_sets(&writes, &[]);
        };

        Ok(Self {
            device,
            start_time: Instant::now(),
            last_frame: Instant::now(),
            fragment_shader_compiler,

            swapchain,
            swapchain_needs_rebuild: false,
            frames_in_flight,

            render_pass,
            framebuffers,

            pipeline,
            pipeline_layout,
            descriptor_set_layout,
            _descriptor_pool: descriptor_pool,
            descriptor_sets,
            uniform_buffer,

            fullscreen_toggle: FullscreenToggle::new(window),
        })
    }

    fn handle_event(
        &mut self,
        window: &mut glfw::Window,
        event: glfw::WindowEvent,
    ) -> Result<()> {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                window.set_should_close(true);
            }
            WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                self.fullscreen_toggle.toggle_fullscreen(window)?;
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, window: &mut glfw::Window) -> Result<()> {
        std::thread::sleep(Duration::from_millis(4));
        if self.swapchain_needs_rebuild {
            self.swapchain_needs_rebuild = false;
            self.rebuild_swapchain(window)?;
        }

        if self.fragment_shader_compiler.check_for_update()? {
            self.frames_in_flight
                .wait_for_all_frames_to_complete()
                .with_context(trace!("Error while waiting for frames"))?;
            let (pipeline, pipeline_layout) = pipeline::create_pipeline(
                &self.device,
                &self.swapchain,
                &self.render_pass,
                &self.descriptor_set_layout,
                self.fragment_shader_compiler.current_shader_bytes(),
            )?;
            self.pipeline = pipeline;
            self.pipeline_layout = pipeline_layout;
        }

        let frame = match self.frames_in_flight.start_frame(&self.swapchain)? {
            FrameStatus::FrameStarted(command_buffer) => command_buffer,
            FrameStatus::SwapchainNeedsRebuild => {
                self.swapchain_needs_rebuild = true;
                return Ok(());
            }
        };

        // write frame data
        {
            let now = Instant::now();
            let dt = (now - self.last_frame).as_secs_f32();
            self.last_frame = now;

            let (x_64, y_64) = window.get_cursor_pos();
            let (w_i32, h_i32) = window.get_size();

            let (x, y) = (x_64 as f32, y_64 as f32);
            let (w, h) = (w_i32 as f32, h_i32 as f32);
            let a = w / h;

            let frame_data = FrameData {
                mouse_pos: [
                    sts::map(x, 0.0..w, -a..a),
                    sts::map(y, 0.0..h, 1.0..-1.0),
                ],
                screen_size: [w, h],
                time: (now - self.start_time).as_secs_f32(),
                dt,
            };
            unsafe {
                self.uniform_buffer
                    .write_indexed(frame.frame_index(), frame_data)?;
            }
        }

        unsafe {
            let clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            };
            self.device.cmd_begin_render_pass(
                frame.command_buffer(),
                &vk::RenderPassBeginInfo {
                    render_pass: self.render_pass.raw,
                    framebuffer: self.framebuffers
                        [frame.swapchain_image_index() as usize]
                        .raw,
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: self.swapchain.extent,
                    },
                    clear_value_count: 1,
                    p_clear_values: &clear_value,
                    ..Default::default()
                },
                vk::SubpassContents::INLINE,
            );

            self.device.cmd_bind_pipeline(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.raw,
            );

            self.device.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                0,
                &[self.descriptor_sets[frame.frame_index()]],
                &[],
            );

            self.device.cmd_draw(frame.command_buffer(), 6, 1, 0, 0);

            self.device.cmd_end_render_pass(frame.command_buffer());
        }

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

/// Create a renderpass for the application.
///
/// The renderpass has a single subpass with a single color attachment for the
/// swapchain image.
fn create_renderpass(
    device: &Device,
    swapchain: &Swapchain,
) -> Result<raii::RenderPass> {
    let attachment_description = vk::AttachmentDescription {
        format: swapchain.format.format,
        samples: vk::SampleCountFlags::TYPE_1,
        load_op: vk::AttachmentLoadOp::CLEAR,
        store_op: vk::AttachmentStoreOp::STORE,
        stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
        initial_layout: vk::ImageLayout::UNDEFINED,
        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        ..Default::default()
    };
    let attachment_reference = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };
    let subpass_description = vk::SubpassDescription {
        pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
        input_attachment_count: 0,
        p_input_attachments: std::ptr::null(),
        color_attachment_count: 1,
        p_color_attachments: &attachment_reference,
        ..Default::default()
    };
    let subpass_dependency = vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        dst_subpass: 0,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask: vk::AccessFlags::empty(),
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        dependency_flags: vk::DependencyFlags::empty(),
    };
    let create_info = vk::RenderPassCreateInfo {
        attachment_count: 1,
        p_attachments: &attachment_description,
        subpass_count: 1,
        p_subpasses: &subpass_description,
        dependency_count: 1,
        p_dependencies: &subpass_dependency,
        ..Default::default()
    };
    raii::RenderPass::new(device.logical_device.clone(), &create_info)
}

/// Creates one framebuffer per swapchain image view.
///
/// Framebuffers must be replaced when the swapchain is rebuilt.
fn create_framebuffers(
    device: &Device,
    render_pass: &raii::RenderPass,
    swapchain: &Swapchain,
) -> Result<Vec<raii::Framebuffer>> {
    let mut framebuffers = vec![];
    let vk::Extent2D { width, height } = swapchain.extent;
    for image_view in &swapchain.image_views {
        let create_info = vk::FramebufferCreateInfo {
            render_pass: render_pass.raw,
            attachment_count: 1,
            p_attachments: &image_view.raw,
            width,
            height,
            layers: 1,
            ..Default::default()
        };
        framebuffers.push(raii::Framebuffer::new(
            device.logical_device.clone(),
            &create_info,
        )?);
    }
    Ok(framebuffers)
}

pub fn main() {
    app_main::<Example>();
}
