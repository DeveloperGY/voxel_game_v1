use crate::gpu::GpuContext;
use crate::render::Renderable;
use crate::render::pipelines::{FakePipeline, FakeVertex};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, IndexFormat, RenderPass, TextureFormat};

pub struct Quad {
    pipeline: FakePipeline,
    vertices: Buffer,
    indices: Buffer,
}

impl Quad {
    pub fn new(gpu_ctx: &GpuContext, surface_format: TextureFormat) -> Self {
        let shader = gpu_ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("fake_quad.wgsl"));
        let pipeline = FakePipeline::new(gpu_ctx, &shader, surface_format);
        let (vertices, indices) = Self::buffers(gpu_ctx);

        Self {
            pipeline,
            vertices,
            indices,
        }
    }

    fn buffers(gpu_ctx: &GpuContext) -> (Buffer, Buffer) {
        let tl = FakeVertex::new(-0.5, 0.5, 1.0);
        let tr = FakeVertex::new(0.5, 0.5, 1.0);
        let bl = FakeVertex::new(-0.5, -0.5, 1.0);
        let br = FakeVertex::new(0.5, -0.5, 1.0);
        let vertices = [tl, tr, bl, br];
        let indices: [u16; 6] = [0, 2, 1, 2, 3, 1];

        let vertex_buffer = gpu_ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = gpu_ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        (vertex_buffer, index_buffer)
    }
}

impl Renderable for Quad {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        render_pass.set_vertex_buffer(0, self.vertices.slice(..));
        render_pass.set_index_buffer(self.indices.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}
