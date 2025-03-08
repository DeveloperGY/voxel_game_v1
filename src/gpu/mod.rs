use wgpu::{Device, Queue};

pub struct GpuContext {
    pub device: Device,
    pub queue: Queue,
}

impl GpuContext {
    pub fn new(device: Device, queue: Queue) -> Self {
        Self { device, queue }
    }
}
