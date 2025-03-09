use crate::gpu::GpuContext;
use crate::render::Renderable;
use crate::world::quad::Quad;
use wgpu::{RenderPass, TextureFormat};

mod quad;

pub struct World {
    fake_quad: Quad,
}

impl World {
    pub fn new(gpu_ctx: &GpuContext, surface_fmt: TextureFormat) -> Self {
        Self {
            fake_quad: Quad::new(gpu_ctx, surface_fmt),
        }
    }
}

impl Renderable for World {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        self.fake_quad.draw(render_pass);
    }
}
