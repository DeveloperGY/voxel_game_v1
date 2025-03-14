#[derive(Copy, Clone)]
pub struct CameraMovementBuffer {
    pub speed: f32,
    pub forward: f32,
    pub backward: f32,
    pub left: f32,
    pub right: f32,
    pub up: f32,
    pub down: f32,
    pub rotate: (f32, f32)
}

impl CameraMovementBuffer {
    pub fn new() -> Self {
        Self {
            speed: 50.0,
            forward: 0.0,
            backward: 0.0,
            left: 0.0,
            right: 0.0,
            up: 0.0,
            down: 0.0,
            rotate: (0.0, 0.0)
        }
    }

    pub fn reset(&mut self) {
        self.forward = 0.0;
        self.backward = 0.0;
        self.left = 0.0;
        self.right = 0.0;
        self.up = 0.0;
        self.down = 0.0;
        self.rotate = (0.0, 0.0);
    }
}