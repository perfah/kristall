use std::time::Duration;
use winit::event::{DeviceEvent, ElementState, VirtualKeyCode, KeyboardInput};
use cgmath::Vector3;

use crate::world::entity::Entity;
use crate::world::entity::component::transform::Transform;
use crate::world::entity::component::rigid_body::RigidBody;

pub trait EntityController: Send + Sync {
    fn update_entity(&self, entity: &Entity, delta: Duration);
    fn on_incoming_event(&mut self, incoming_event: &DeviceEvent) -> bool;
}

pub enum InputAccelerationMethod {
    Force(f32),
    Velocity(f32)
}

pub struct WASDEntityController {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    acc_method: InputAccelerationMethod
}

impl WASDEntityController {
    pub fn new(acc_method: InputAccelerationMethod) -> WASDEntityController{
        WASDEntityController { 
            forward: false,
            back: false,
            left: false,
            right: false,
            up: false,
            down: false,
            acc_method
        }
    }
}

impl EntityController for WASDEntityController {
    fn update_entity(&self, entity: &Entity, _delta: Duration) {
        let dir_x = if self.left { -1f32 } else if self.right { 1f32 } else { 0f32 };
        let dir_y = if self.down { -1f32 } else if self.up { 1f32 } else { 0f32 };
        let dir_z = if self.forward { -1f32 } else if self.back { 1f32 } else { 0f32 };
        
        if let Some(mgr) = entity.component::<RigidBody>() {
            let mut rigid_body = mgr.lock_component_for_write();

            match self.acc_method {
                InputAccelerationMethod::Force(magnitude) => {
                    rigid_body.commit_force("input", Vector3::<f32> { 
                        x: dir_x * magnitude, 
                        y: dir_y * magnitude,
                        z: dir_z * magnitude
                    });
                    
                },
                InputAccelerationMethod::Velocity(vel) => {
                    let RigidBody { ref mut velocity, .. } = *rigid_body;

                    velocity.x = dir_x * vel;
                    velocity.y = dir_y * vel;
                    velocity.z = dir_z * vel;
                }
            }
        }
        else {
            println!("Warning: Expected 'RigidBody' component as controller is configured to use InputAccelerationMethod::Force.")
        }

    
    }

    fn on_incoming_event(&mut self, incoming_event: &DeviceEvent) -> bool {
        match incoming_event {
            DeviceEvent::Key (KeyboardInput {
                state,
                virtual_keycode: Some(keycode),
                ..
            }) => {
                let pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W => { self.forward = pressed; true },
                    VirtualKeyCode::S => { self.back = pressed; true },
                    VirtualKeyCode::A => { self.left = pressed; true },
                    VirtualKeyCode::D => { self.right = pressed; true },
                    VirtualKeyCode::Space => { self.up = pressed; true },
                    VirtualKeyCode::LShift => { self.down = pressed; true }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}
