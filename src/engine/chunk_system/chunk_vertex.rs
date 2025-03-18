use crate::engine::gpu::Vertex;
use bytemuck::{Pod, Zeroable};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ChunkVertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}

impl ChunkVertex {
    const ATTRIBS: [VertexAttribute; 3] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];
}

impl Vertex for ChunkVertex {
    fn layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            attributes: &Self::ATTRIBS,
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
        }
    }
}
