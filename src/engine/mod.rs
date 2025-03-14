mod chunk_system;
mod gpu;
mod input_system;
mod render_system;

use crate::engine::chunk_system::ChunkSystem;
use crate::engine::render_system::RenderSystem;
use std::sync::Arc;
use std::time::Instant;
use winit::event::{KeyEvent, WindowEvent};
use winit::window::Window;
use crate::engine::input_system::InputSystem;

pub struct Engine {
    window: Arc<Window>,
    render_system: RenderSystem,
    chunk_system: ChunkSystem,
    input_system: InputSystem,
    now: Instant
}

impl Engine {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let render_system = RenderSystem::new(Arc::clone(&window));
        let chunk_system = ChunkSystem::new(render_system.get_gpu_ctx());
        let input_system = InputSystem::new();
        Self {
            window,
            render_system,
            chunk_system,
            input_system,
            now: Instant::now()
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.render_system.resize(width, height);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn handle_key_input(&mut self, event: KeyEvent) {
        self.input_system.handle_key_event(event);
    }
    
    pub fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.input_system.handle_mouse_move(x as f32, y as f32);
    }
    
    pub fn window_focus(&mut self, flag: bool) {
        self.window.set_cursor_visible(!flag);
    }

    pub fn draw_frame(&mut self) {
        let dt = self.now.elapsed();
        self.now = Instant::now();
        
        let movement = self.input_system.get_movement();
        self.render_system.move_camera(movement, dt);
        self.render_system.render(&self.chunk_system);
        
        let fps = 1.0 / dt.as_secs_f64();
        println!("FPS: {}, Render Frametime: {}ms", fps.trunc(), dt.as_millis());
    }
}
