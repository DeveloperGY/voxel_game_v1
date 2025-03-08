use crate::window_handler::WindowHandler;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn launch() {
    let mut window_handler = WindowHandler::new();

    let event_loop = EventLoop::new().expect("Failed to create winit event loop!");
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run_app(&mut window_handler)
        .expect("Application crashed unexpectedly!");
}
