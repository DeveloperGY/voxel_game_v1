use crate::engine::gpu::GpuCtx;
use pollster::FutureExt;
use std::rc::Rc;
use std::sync::Arc;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PowerPreference, PresentMode,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
    SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
use winit::window::Window;

mod renderable;
pub use renderable::Renderable;

async fn initialize_wgpu(window: Arc<Window>) -> (GpuCtx, Surface<'static>, SurfaceConfiguration) {
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
                required_features: Features::POLYGON_MODE_LINE,
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

    let gpu_ctx = GpuCtx::new(device, queue, format);
    (gpu_ctx, surface, surface_config)
}

pub struct RenderSystem {
    gpu_ctx: Rc<GpuCtx>,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    window: Arc<Window>,
}

impl RenderSystem {
    pub fn new(window: Arc<Window>) -> Self {
        let (gpu_ctx, surface, surface_config) = initialize_wgpu(Arc::clone(&window)).block_on();
        Self {
            gpu_ctx: Rc::new(gpu_ctx),
            surface,
            surface_config,
            window,
        }
    }

    pub fn get_gpu_ctx(&self) -> Rc<GpuCtx> {
        Rc::clone(&self.gpu_ctx)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface
                .configure(&self.gpu_ctx.device, &self.surface_config);
        }
    }

    pub fn render(&self, renderable: &impl Renderable) {
        let target = match self.surface.get_current_texture() {
            Ok(target) => target,
            _ => return,
        };

        let view = target
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .gpu_ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            renderable.render(&mut pass);
        }

        self.gpu_ctx.queue.submit(std::iter::once(encoder.finish()));
        self.window.pre_present_notify();
        target.present();
    }
}
