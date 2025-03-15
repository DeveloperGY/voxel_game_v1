use cgmath::{perspective, Deg, Matrix4};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0
);

pub struct PerspectiveProjection {
    aspect: f32,
    fov_y_deg: f32,
    z_near: f32,
    z_far: f32
}

impl PerspectiveProjection {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fov_y_deg: 60.0,
            z_near: 0.01,
            z_far: 100.0
        }
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
    
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(Deg(self.fov_y_deg), self.aspect, self.z_near, self.z_far)
    }
}