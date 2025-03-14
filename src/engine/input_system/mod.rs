use std::collections::HashMap;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::engine::gpu::CameraMovementBuffer;

pub struct InputSystem {
    camera_movement_buffer: CameraMovementBuffer,
    states: HashMap<KeyCode, bool>
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            camera_movement_buffer: CameraMovementBuffer::new(),
            states: HashMap::new()
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {physical_key, state, ..} => {
                match physical_key {
                    PhysicalKey::Code(code) => {
                        self.states.insert(code, state.is_pressed());
                    }
                    _ => ()
                }
            }
            _ => ()
        };
    }

    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        self.camera_movement_buffer.rotate.0 += x;
        self.camera_movement_buffer.rotate.1 -= y;
    }

    pub fn get_movement(&mut self) -> CameraMovementBuffer {
        if *self.states.get(&KeyCode::KeyW).unwrap_or(&false) {
            self.camera_movement_buffer.forward = 1.0;
        }
        if *self.states.get(&KeyCode::KeyS).unwrap_or(&false) {
            self.camera_movement_buffer.backward = 1.0;
        }
        if *self.states.get(&KeyCode::KeyA).unwrap_or(&false) {
            self.camera_movement_buffer.left = 1.0;
        }
        if *self.states.get(&KeyCode::KeyD).unwrap_or(&false) {
            self.camera_movement_buffer.right = 1.0;
        }
        if *self.states.get(&KeyCode::ControlLeft).unwrap_or(&false) {
            self.camera_movement_buffer.down = 1.0;
        }
        if *self.states.get(&KeyCode::Space).unwrap_or(&false) {
            self.camera_movement_buffer.up = 1.0;
        }

        let buffer = self.camera_movement_buffer;
        self.camera_movement_buffer.reset();
        buffer
    }
}