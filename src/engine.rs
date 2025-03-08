use crate::gpu::GpuContext;
use crate::renderer::Renderer;
use pollster::FutureExt;
use std::sync::Arc;
use winit::window::Window;

pub struct Engine {
    window: Arc<Window>,
    renderer: Renderer,
    gpu_context: GpuContext,
}

impl Engine {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let (renderer, gpu_context) = Renderer::new(Arc::clone(&window)).block_on();
        Self {
            window,
            renderer,
            gpu_context,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(&self.gpu_context, width, height);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn update(&self) {
        self.renderer.render(&self.gpu_context);
    }
}
