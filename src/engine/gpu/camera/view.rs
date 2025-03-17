use crate::engine::gpu::camera::CameraMovementBuffer;
use cgmath::num_traits::FloatConst;
use cgmath::{Angle, InnerSpace, Matrix4, Point3, Rad, Vector3};
use cgmath::{Deg, Zero};
use std::f32::consts::PI;
use std::time::Duration;

pub struct View {
    pos: Point3<f32>,
    pitch: Rad<f32>,
    yaw: Rad<f32>,
}

impl View {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(0.0, 0.0, 0.0),
            pitch: Rad(0.0),
            yaw: Rad(f32::PI() / 2.0),
        }
    }

    pub fn move_camera(&mut self, buffer: CameraMovementBuffer, dt: Duration) {
        let dt = dt.as_secs_f32();
        let sensitivity = 1.0;

        let (rot_yaw, rot_pitch) = buffer.rotate;
        self.yaw += Rad(rot_yaw * dt * sensitivity);
        self.pitch += Rad(rot_pitch * dt * sensitivity);

        if self.pitch > Deg(89.0).into() {
            self.pitch = Deg(89.0).into();
        } else if self.pitch < Deg(-89.0).into() {
            self.pitch = Deg(-89.0).into();
        }

        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();

        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos);
        let up = Vector3::unit_y();

        let z = buffer.forward - buffer.backward;
        let x = buffer.right - buffer.left;
        let y = buffer.up - buffer.down;

        let z = forward * z;
        let x = right * x;
        let y = up * y;

        let xz = match z + x {
            zeroed if zeroed == Vector3::zero() => zeroed,
            xz => xz.normalize(),
        };

        self.pos += buffer.speed * (xz + y) * dt;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (pitch_sin, pitch_cos) = self.pitch.sin_cos();
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();

        Matrix4::look_to_rh(
            self.pos,
            Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin),
            Vector3::unit_y(),
        )
    }
}
