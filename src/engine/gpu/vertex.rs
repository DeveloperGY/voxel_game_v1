use bytemuck::Pod;
use wgpu::VertexBufferLayout;

pub trait Vertex: Pod {
    fn layout<'a>() -> VertexBufferLayout<'a>;
}
