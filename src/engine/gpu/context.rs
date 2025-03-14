use wgpu::{Device, Queue, TextureFormat};

pub struct GpuCtx {
    pub device: Device,
    pub queue: Queue,
    pub surface_format: TextureFormat,
}

impl crate::engine::gpu::GpuCtx {
    pub fn new(device: Device, queue: Queue, surface_format: TextureFormat) -> Self {
        Self {
            device,
            queue,
            surface_format,
        }
    }
}
