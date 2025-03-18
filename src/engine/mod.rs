mod chunk_system;
mod gpu;
mod input_system;
mod render_system;
pub mod utils;

use crate::engine::chunk_system::{ChunkSystem, ThreadedChunkLoader};
use crate::engine::input_system::InputSystem;
use crate::engine::render_system::RenderSystem;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::event::KeyEvent;
use winit::window::{CursorGrabMode, Window};

pub struct Engine {
    window: Arc<Window>,
    render_system: RenderSystem,
    chunk_system: ChunkSystem<ThreadedChunkLoader>,
    input_system: InputSystem,
    prev_now: Instant,
    accumulated_dt: Duration
}

impl Engine {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);

        let render_system = RenderSystem::new(Arc::clone(&window));

        let chunk_loader = ThreadedChunkLoader::new(render_system.get_gpu_ctx());
        let chunk_system = ChunkSystem::new(render_system.get_gpu_ctx(), chunk_loader);

        let input_system = InputSystem::new();

        Self {
            window,
            render_system,
            chunk_system,
            input_system,
            prev_now: Instant::now(),
            accumulated_dt: Duration::ZERO
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
        if !flag {
            self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
        } else {
            self.window
                .set_cursor_grab(CursorGrabMode::Confined)
                .unwrap();
        }
    }

    pub fn run_frame(&mut self) {
        // Calculate delta time
        let now = Instant::now();
        let dt = now.duration_since(self.prev_now);
        self.prev_now = Instant::now();
        self.accumulated_dt += dt;

        // print fps
        let fps = 1.0 / dt.as_secs_f64();
        println!(
            "FPS: {}, Render Frametime: {}ms",
            fps.trunc(),
            dt.as_millis()
        );

        // Run fixed time step
        let fixed_time_step = Duration::from_secs_f32(1.0 / 60.0);
        while self.accumulated_dt >= fixed_time_step {
            println!("Fixed Frame!");
            let movement = self.input_system.get_movement();
            self.render_system.move_camera(movement, fixed_time_step);

            let (p_x, _, p_z) = self.render_system.get_camera_pos();
            self.chunk_system.player_moved(p_x.trunc() as i32, p_z.trunc() as i32);

            self.accumulated_dt -= fixed_time_step;
        }

        // Run frame step
        self.chunk_system.handle_chunk_jobs();
        self.render_system.render(&self.chunk_system);
    }
}
