use crate::gpu::GpuContext;
use crate::render::Renderable;
use crate::render::mesh::{CpuMesh, GpuMesh};
use crate::render::pipelines::{FakePipeline, FakeVertex};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, IndexFormat, RenderPass, TextureFormat};

pub struct Quad {
    pipeline: FakePipeline,
    mesh: GpuMesh,
}

impl Quad {
    pub fn new(gpu_ctx: &GpuContext, surface_format: TextureFormat) -> Self {
        let shader = gpu_ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("fake_quad.wgsl"));
        let pipeline = FakePipeline::new(gpu_ctx, &shader, surface_format);
        let mesh = Self::mesh(gpu_ctx);

        Self { pipeline, mesh }
    }

    fn mesh(gpu_ctx: &GpuContext) -> GpuMesh {
        let mut cpu_mesh = CpuMesh::new();
        cpu_mesh.push_vertex(FakeVertex::new(-0.5, 0.5, 1.0));
        cpu_mesh.push_vertex(FakeVertex::new(0.5, 0.5, 1.0));
        cpu_mesh.push_vertex(FakeVertex::new(-0.5, -0.5, 1.0));
        cpu_mesh.push_vertex(FakeVertex::new(0.5, -0.5, 1.0));

        cpu_mesh.push_index(0);
        cpu_mesh.push_index(2);
        cpu_mesh.push_index(1);
        cpu_mesh.push_index(2);
        cpu_mesh.push_index(3);
        cpu_mesh.push_index(1);

        unsafe { cpu_mesh.try_into_gpu_mesh(gpu_ctx).unwrap_unchecked() }
    }
}

impl Renderable for Quad {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        self.mesh.draw(render_pass);
    }
}
