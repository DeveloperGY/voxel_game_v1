use std::sync::Arc;
use winit::window::Window;

pub struct Engine {
    window: Arc<Window>,
}

impl Engine {
    pub fn new(window: Window) -> Self {
        Self {
            window: Arc::new(window),
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
