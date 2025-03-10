use crate::render::Renderable;
use wgpu::{Buffer, IndexFormat, RenderPass};

pub struct GpuMesh {
    vertices: Buffer,
    indices: Buffer,
    index_count: u32,
}

impl GpuMesh {
    pub fn new(vertices: Buffer, indices: Buffer, index_count: u32) -> Self {
        Self {
            vertices,
            indices,
            index_count,
        }
    }
}

impl Renderable for GpuMesh {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertices.slice(..));
        render_pass.set_index_buffer(self.indices.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
