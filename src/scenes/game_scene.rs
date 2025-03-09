use crate::gpu::GpuContext;
use crate::render::{Renderable, Scene};
use crate::world::World;
use wgpu::{RenderPass, TextureFormat};

pub struct GameScene {
    world: World,
}

impl GameScene {
    pub fn new(gpu_ctx: &GpuContext, surface_format: TextureFormat) -> Self {
        Self {
            world: World::new(gpu_ctx, surface_format),
        }
    }
}

impl Renderable for GameScene {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        self.world.draw(render_pass);
    }
}

impl Scene for GameScene {}
