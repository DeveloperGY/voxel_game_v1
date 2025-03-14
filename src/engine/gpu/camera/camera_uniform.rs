use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4]
}

impl CameraUniform {
    pub fn from_matrices(view_matrix: cgmath::Matrix4<f32>, perspective_matrix: cgmath::Matrix4<f32>) -> Self {
        let view_proj = perspective_matrix * view_matrix;
        
        Self {
            view_proj: view_proj.into()
        }
    }
}