mod chunk_system;
mod gpu;
mod input_system;
mod render_system;

use crate::engine::chunk_system::ChunkSystem;
use crate::engine::render_system::RenderSystem;
use std::sync::Arc;
use winit::window::Window;

pub struct Engine {
    window: Arc<Window>,
    render_system: RenderSystem,
    chunk_system: ChunkSystem,
}

impl Engine {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let render_system = RenderSystem::new(Arc::clone(&window));
        let chunk_system = ChunkSystem::new(render_system.get_gpu_ctx());
        Self {
            window,
            render_system,
            chunk_system,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.render_system.resize(width, height);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn handle_input(&self) {
        // TODO: gak
    }

    pub fn draw_frame(&self) {
        let start = std::time::Instant::now();
        self.render_system.render(&self.chunk_system);
        let elapsed = start.elapsed();
        println!("Render Frametime: {}ms", elapsed.as_millis());
        let fps = 1.0 / elapsed.as_secs_f64();
        println!("FPS: {}", fps.trunc());
    }
}
