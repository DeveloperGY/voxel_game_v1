use wgpu::VertexBufferLayout;

pub trait Vertex {
    fn layout<'a>() -> VertexBufferLayout<'a>;
}
