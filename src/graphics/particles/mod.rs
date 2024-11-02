mod view;

use {
    crate::graphics::vulkan::{Device, FramesInFlight},
    anyhow::Result,
    std::sync::Arc,
    view::ParticlesView,
};

#[derive(Debug)]
pub struct Particles {
    view: ParticlesView,
}

impl Particles {
    pub fn new(
        device: Arc<Device>,
        frames_in_flight: &FramesInFlight,
    ) -> Result<Self> {
        let view = ParticlesView::new(device, frames_in_flight)?;
        Ok(Self { view })
    }
}
