use crate::engine::gpu::{Camera, CameraMovementBuffer, GpuCtx};
use pollster::FutureExt;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use wgpu::{
    AddressMode, Backends, Color, CommandEncoderDescriptor, CompareFunction, DeviceDescriptor,
    Extent3d, Features, FilterMode, Instance, InstanceDescriptor, Limits, LoadOp, MemoryHints,
    Operations, PowerPreference, PresentMode, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RequestAdapterOptions, Sampler,
    SamplerDescriptor, StoreOp, Surface, SurfaceConfiguration, Texture, TextureDescriptor,
    TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
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

fn create_depth_texture(
    gpu_ctx: &GpuCtx,
    width: u32,
    height: u32,
) -> (Texture, TextureView, Sampler) {
    let size = Extent3d {
        width: width.max(1),
        height: height.max(1),
        depth_or_array_layers: 1,
    };

    let desc = TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Depth32Float,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };
    let texture = gpu_ctx.device.create_texture(&desc);
    let view = texture.create_view(&TextureViewDescriptor::default());
    let sampler = gpu_ctx.device.create_sampler(&SamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        mipmap_filter: FilterMode::Nearest,
        compare: Some(CompareFunction::LessEqual),
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        ..Default::default()
    });
    (texture, view, sampler)
}

pub struct RenderSystem {
    gpu_ctx: Arc<GpuCtx>,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    window: Arc<Window>,
    camera: Camera,
    depth_texture: Texture,
    depth_texture_view: TextureView,
    depth_sampler: Sampler,
}

impl RenderSystem {
    pub fn new(window: Arc<Window>) -> Self {
        let (gpu_ctx, surface, surface_config) = initialize_wgpu(Arc::clone(&window)).block_on();
        let width = surface_config.width;
        let height = surface_config.height;
        let camera = Camera::new(&gpu_ctx, width, height);
        let (depth_texture, depth_texture_view, depth_sampler) =
            create_depth_texture(&gpu_ctx, width, height);

        Self {
            gpu_ctx: Arc::new(gpu_ctx),
            surface,
            surface_config,
            window,
            camera,
            depth_texture,
            depth_texture_view,
            depth_sampler,
        }
    }

    pub fn get_gpu_ctx(&self) -> Arc<GpuCtx> {
        Arc::clone(&self.gpu_ctx)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface
                .configure(&self.gpu_ctx.device, &self.surface_config);
            self.camera.resize(width, height);
            let (depth_texture, depth_texture_view, depth_sampler) =
                create_depth_texture(&self.gpu_ctx, width, height);
            self.depth_texture = depth_texture;
            self.depth_texture_view = depth_texture_view;
            self.depth_sampler = depth_sampler;
        }
    }

    pub fn move_camera(&mut self, movement: CameraMovementBuffer, dt: Duration) {
        self.camera.move_camera(movement, dt);
    }

    pub fn render(&self, renderable: &impl Renderable) {
        self.camera.update_buffer(&self.gpu_ctx);

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
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_bind_group(0, self.camera.bind_group(), &[]);
            renderable.render(&mut pass);
        }

        self.gpu_ctx.queue.submit(std::iter::once(encoder.finish()));
        self.window.pre_present_notify();
        target.present();
    }
}
