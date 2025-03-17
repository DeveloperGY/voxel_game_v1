use wgpu::RenderPass;

pub trait Renderable {
    fn render(&self, pass: &mut RenderPass);
}
