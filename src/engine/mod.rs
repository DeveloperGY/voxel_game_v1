use crate::gpu::GpuContext;
use crate::render::Renderer;
use crate::scenes::GameScene;
use pollster::FutureExt;
use std::sync::Arc;
use winit::window::Window;

pub struct Engine {
    window: Arc<Window>,
    renderer: Renderer,
    gpu_context: GpuContext,
    scene: GameScene,
}

impl Engine {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let (renderer, gpu_context) = Renderer::new(Arc::clone(&window)).block_on();
        let scene = GameScene::new(&gpu_context, renderer.config().format);
        Self {
            window,
            renderer,
            gpu_context,
            scene,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(&self.gpu_context, width, height);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn update(&self) {
        let start = std::time::Instant::now();
        self.renderer.render(&self.gpu_context, &self.scene);
        let elapsed = start.elapsed();
        let fps = 1.0 / elapsed.as_secs_f64();
        let frame_time = elapsed.as_millis();

        println!("FPS: {:>5}, Frame Time: {}ms", fps.trunc(), frame_time);
    }
}
