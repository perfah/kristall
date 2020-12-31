
use crate::backend::graphics::OPENGL_TO_WGPU_MATRIX;
use winit::event::{WindowEvent, ElementState, VirtualKeyCode, KeyboardInput};
use cgmath::{Point3, Vector3, EuclideanSpace, InnerSpace};
use crate::world::entity::component::{Component, ComponentManager};
use crate::world::entity::component::transform::Transform;

pub enum CameraPerspective {
    FirstPersonView,
    ThirdPersonView{
        target: ComponentManager<Transform>,
        distance: f32,
        angle_xz: f32,
        angle_xy: f32
    }
}

pub struct Camera {
    pub perspective: CameraPerspective,
    pub controller: CameraController,
    //pub eye: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new_third_person(target: ComponentManager<Transform>) -> Camera {
        Camera {
            perspective: CameraPerspective::ThirdPersonView{
                target,
                distance: 25f32,
                angle_xz: 0f32,
                angle_xy: 0f32
            },
            controller: CameraController::new(0.2),
            //eye: (0.0, 5.0, -10.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 10 as f32 / 10 as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        match &self.perspective {
            CameraPerspective::ThirdPersonView { target, distance, angle_xz, angle_xy }=> {
                let target = target.peek(|transform| Point3::from_vec(transform.pos)).unwrap();

                let tot = 2.0 * std::f32::consts::PI;

                let eye = target - (Vector3::new(
                    f32::cos(angle_xz % tot) * f32::cos(angle_xy % tot),
                    f32::sin(angle_xy % tot),
                    f32::sin(angle_xz % tot) * f32::cos(angle_xy % tot)
                ).normalize() * (*distance));

                let view = cgmath::Matrix4::look_at(
                    eye, 
                    target, 
                    if f32::cos(angle_xy % tot) > 0f32 { self.up } else { -self.up});
                let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
                return OPENGL_TO_WGPU_MATRIX * proj * view;
            },
            _ => unimplemented!()
        }
    }

    pub fn update(&mut self) {
        match self.perspective {
            CameraPerspective::ThirdPersonView {ref target, ref mut distance, ref mut angle_xz, ref mut angle_xy} => {
                let target = target.peek(|transform| Point3::from_vec(transform.pos)).unwrap();

                //let forward = target - self.eye;
                //let forward_norm = forward.normalize();
                //let forward_mag = forward.magnitude();

                // Prevents glitching when camera gets too close to the
                // center of the scene.
                if self.controller.is_forward_pressed { //&& forward_mag > self.controller.speed
                    //self.eye += forward_norm * self.controller.speed;
                    *angle_xy -= 0.05f32;
                }
                if self.controller.is_backward_pressed {
                    //self.eye -= forward_norm * self.controller.speed;
                    *angle_xy += 0.05f32;
                }

                //let right = forward_norm.cross(self.up);

                // Redo radius calc in case the up/ down is pressed.
                //let forward = target - self.eye;
                //let forward_mag = forward.magnitude();

                if self.controller.is_right_pressed {
                    // Rescale the distance between the target and eye so
                    // that it doesn't change. The eye therefore still
                    // lies on the circle made by the target and eye.
                    //self.eye = target - (forward + right * self.controller.speed).normalize() * forward_mag;
                    *angle_xz += 0.05f32;
                }
                if self.controller.is_left_pressed {
                    //self.eye = target - (forward - right * self.controller.speed).normalize() * forward_mag;
                    *angle_xz -= 0.05f32;
                }

                if self.controller.is_up_pressed {
                    //self.eye += cgmath::Vector3::unit_y() * 0.5;
                    *distance += 0.05f32;
                }

                if self.controller.is_down_pressed {
                    //self.eye -= cgmath::Vector3::unit_y() * 0.5;
                    *distance -= 0.05f32;
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
                    VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
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

impl Component for Camera {
    fn enabled(&self) -> bool {
        unimplemented!()
    }
}





