use std::time::Duration;
use cgmath::{Angle, InnerSpace};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, ShaderStages};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::engine::gpu::camera::camera_uniform::CameraUniform;
use crate::engine::gpu::camera::perspective::PerspectiveProjection;
use crate::engine::gpu::camera::view::View;
use crate::engine::gpu::GpuCtx;

mod camera_uniform;
mod perspective;
mod view;
mod camera_movement_buffer;
pub use camera_movement_buffer::CameraMovementBuffer;


pub struct Camera {
    view: View,
    perspective: PerspectiveProjection,
    uniform_buffer: Buffer,
    bind_group: BindGroup
}

impl Camera {
    pub fn new(gpu_ctx: &GpuCtx, width: u32, height: u32) -> Self {
        let view = View::new();
        let perspective = PerspectiveProjection::new(width, height);
        let uniform_data = CameraUniform::from_matrices(view.calc_matrix(), perspective.calc_matrix());

        let uniform_buffer = gpu_ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let bind_group_layout = gpu_ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                binding: 0,
                visibility: ShaderStages::VERTEX,
                count: None
            }]
        });

        let bind_group = gpu_ctx.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding()
            }]
        });

        Self {
            view,
            perspective,
            uniform_buffer,
            bind_group
        }
    }
    
    pub fn move_camera(&mut self, movement: CameraMovementBuffer, dt: Duration) {
        self.view.move_camera(movement, dt);
    }
    
    pub fn resize(&mut self, width: u32, height: u32) {
        self.perspective.resize(width, height);
    }

    pub fn update_buffer(&self, gpu_ctx: &GpuCtx) {
        let uniform_data = CameraUniform::from_matrices(self.view.calc_matrix(), self.perspective.calc_matrix());
        gpu_ctx.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform_data]));
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}