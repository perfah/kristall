use crate::backend::graphics::OPENGL_TO_WGPU_MATRIX;
use winit::event::{WindowEvent, ElementState, VirtualKeyCode, KeyboardInput};
use cgmath::InnerSpace;

pub enum CameraPerspective {
    FirstPersonView,
    ThirdPersonView{
        // Change to TransformComponent
        target: cgmath::Point3<f32>,
    }
}

pub struct Camera {
    pub perspective: CameraPerspective,
    pub controller: CameraController,
    pub eye: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        match self.perspective {
            CameraPerspective::ThirdPersonView { target }=> {
                let view = cgmath::Matrix4::look_at(self.eye, target, self.up);
                let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
                return OPENGL_TO_WGPU_MATRIX * proj * view;
            },
            _ => unimplemented!()
        }
    }

    pub fn update(&mut self) {
        match self.perspective {
            CameraPerspective::ThirdPersonView {target} => {
                let forward = target - self.eye;
                let forward_norm = forward.normalize();
                let forward_mag = forward.magnitude();

                // Prevents glitching when camera gets too close to the
                // center of the scene.
                if self.controller.is_forward_pressed && forward_mag > self.controller.speed {
                    self.eye += forward_norm * self.controller.speed;
                }
                if self.controller.is_backward_pressed {
                    self.eye -= forward_norm * self.controller.speed;
                }

                let right = forward_norm.cross(self.up);

                // Redo radius calc in case the up/ down is pressed.
                let forward = target - self.eye;
                let forward_mag = forward.magnitude();

                if self.controller.is_right_pressed {
                    // Rescale the distance between the target and eye so
                    // that it doesn't change. The eye therefore still
                    // lies on the circle made by the target and eye.
                    self.eye = target - (forward + right * self.controller.speed).normalize() * forward_mag;
                }
                if self.controller.is_left_pressed {
                    self.eye = target - (forward - right * self.controller.speed).normalize() * forward_mag;
                }
            },
            _ => {}
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        self.controller.process_events(event)
    }
}

pub struct CameraController {
    pub speed: f32,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}





