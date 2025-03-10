use crate::gpu::GpuContext;
use crate::render::mesh::GpuMesh;
use crate::render::vertex::Vertex;
use bytemuck::{Pod, Zeroable};
use std::any::Any;
use wgpu::BufferUsages;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub struct CpuMesh<V: Vertex + Pod> {
    vertices: Vec<V>,
    indices: Vec<u16>,
}

impl<V: Vertex + Pod> CpuMesh<V> {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
        }
    }

    pub fn from_data(vertices: &[V], indices: &[u16]) -> Self {
        Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
        }
    }

    pub fn push_vertex(&mut self, vertex: V) {
        self.vertices.push(vertex);
    }

    pub fn push_index(&mut self, index: u16) {
        self.indices.push(index);
    }

    pub fn try_into_gpu_mesh(self, gpu_ctx: &GpuContext) -> Option<GpuMesh> {
        if self.vertices.is_empty() || self.indices.is_empty() {
            return None;
        }

        let index_count = self.indices.len() as u32;

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

        Some(GpuMesh::new(vertex_buffer, index_buffer, index_count))
    }
}
