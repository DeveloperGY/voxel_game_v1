use crate::engine::Engine;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};

pub struct WindowHandler {
    engine: Option<Engine>,
}

impl WindowHandler {
    pub fn new() -> Self {
        Self { engine: None }
    }
}

impl ApplicationHandler for WindowHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = WindowAttributes::default().with_title("Voxel Game V1");
        let window = event_loop
            .create_window(window_attrs)
            .expect("Failed to create window!");

        self.engine = Some(Engine::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // Safety: Engine should be initialized if we have a window to get events from
        let engine = unsafe { self.engine.as_mut().unwrap_unchecked() };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => engine.resize(size.width, size.height),
            WindowEvent::RedrawRequested => engine.draw_frame(),
            _ => (),
        };
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // Safety: Engine should be initialized at this point
        unsafe { self.engine.as_ref().unwrap_unchecked() }.request_redraw();
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        // TODO: Notify engine to shutdown
    }
}
