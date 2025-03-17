use crate::engine::gpu::GpuCtx;
use crate::engine::gpu::vertex::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, RenderPass};

pub struct CpuMesh<V: Vertex> {
    vertices: Vec<V>,
    indices: Vec<u32>,
}

impl<V: Vertex> CpuMesh<V> {
    pub fn new(vertices: Vec<V>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn to_gpu_mesh(&self, gpu_ctx: &GpuCtx) -> Option<GpuMesh> {
        if !self.vertices.is_empty() && !self.indices.is_empty() {
            let vertex_buffer = gpu_ctx.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.vertices),
                usage: BufferUsages::VERTEX,
            });

            let index_buffer = gpu_ctx.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.indices),
                usage: BufferUsages::INDEX,
            });

            Some(GpuMesh {
                vertex_buffer,
                index_buffer,
                index_count: self.indices.len() as u32,
            })
        } else {
            None
        }
    }
}

pub struct GpuMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl GpuMesh {
    pub fn get_vertices(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn get_indices(&self) -> &Buffer {
        &self.index_buffer
    }

    pub fn get_index_count(&self) -> u32 {
        self.index_count
    }
}
