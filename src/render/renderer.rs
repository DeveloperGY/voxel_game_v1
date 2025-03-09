use crate::gpu::GpuContext;
use std::sync::Arc;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PowerPreference, PresentMode,
    RenderPass, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp,
    Surface, SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
use winit::window::Window;

pub trait Renderable {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>);
}

/// A marker trait for the top level renderable
pub trait Scene: Renderable {}

pub struct Renderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> (Self, GpuContext) {
        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        let surface = instance
            .create_surface(Arc::clone(&window))
            .expect("Failed to create window surface!");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to receive gpu adapter!");

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to receive gpu device!");

        let surface_caps = surface.get_capabilities(&adapter);
        let format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            alpha_mode: surface_caps.alpha_modes[0],
            present_mode: PresentMode::AutoNoVsync,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let renderer = Self {
            window,
            surface,
            surface_config,
        };
        let gpu_ctx = GpuContext::new(device, queue);

        (renderer, gpu_ctx)
    }

    pub fn resize(&mut self, gpu_ctx: &GpuContext, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface
                .configure(&gpu_ctx.device, &self.surface_config);
        }
    }

    pub fn render(&self, gpu_ctx: &GpuContext, renderable: &impl Scene) {
        let target = if let Ok(t) = self.surface.get_current_texture() {
            t
        } else {
            return;
        };
        let view = target
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = gpu_ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                    resolve_target: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            renderable.draw(&mut pass);
        }

        gpu_ctx.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        target.present();
    }

    pub fn config(&self) -> &SurfaceConfiguration {
        &self.surface_config
    }
}
