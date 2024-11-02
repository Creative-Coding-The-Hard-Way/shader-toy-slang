mod view;

use {
    super::vulkan::{raii, Swapchain},
    crate::graphics::vulkan::{Device, FramesInFlight},
    anyhow::Result,
    bon::bon,
    std::sync::Arc,
    view::ParticlesView,
};

#[derive(Debug)]
pub struct Particles {
    view: ParticlesView,
}

#[bon]
impl Particles {
    #[builder]
    pub fn new(
        device: Arc<Device>,
        frames_in_flight: &FramesInFlight,
        swapchain: &Swapchain,
        render_pass: &raii::RenderPass,
    ) -> Result<Self> {
        let view = ParticlesView::builder()
            .device(device)
            .frames_in_flight(frames_in_flight)
            .swapchain(swapchain)
            .render_pass(render_pass)
            .build()?;
        Ok(Self { view })
    }
}
