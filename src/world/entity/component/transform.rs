use std::sync::Arc;
use crate::world::entity::component::{Component};
use crate::backend::graphics::transform::{ModelView};
use failure::_core::any::Any;
use cgmath::{Vector3, Quaternion};
use crate::backend::BackendProxy;

pub struct Transform {
    pub pos: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub acc: Vector3<f32>,

    pub rot: Vector3<f32>,
    pub rot_vel: Vector3<f32>,
    pub rot_acc: Vector3<f32>,

    pub frozen: bool,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            pos: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            scale: Vector3 {x: 1.0, y: 1.0, z: 1.0},
            vel: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            acc: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            rot: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            rot_vel: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            rot_acc: Vector3 {x: 0.1, y: 0.1, z: 0.1},
            frozen: false,
        }
    }

    pub fn frozen() -> Transform {
        let mut transform = Transform::new();
        transform.frozen = true;
        transform
    }

    pub fn with_position(mut self, pos: Vector3<f32>) -> Self{
        self.pos = pos;
        self
    }

    pub fn with_velocity(mut self, vel: Vector3<f32>) -> Self{
        self.vel = vel;
        self
    }

    pub fn with_offset(mut self, offset: &Transform) -> Self {
        self.pos += offset.pos;
        self.rot += offset.rot;

        // TODO: Check if this is the way to do it:
        self.scale.x *= offset.scale.x;
        self.scale.y *= offset.scale.y;
        self.scale.z *= offset.scale.z;
        
        self
    }
}

impl Transform {
    pub fn to_raw(&self) -> ModelView {
        unimplemented!()
    }
}

impl Component for Transform {
    fn enabled(&self) -> bool {
        true
    }
}

impl Clone for Transform{
    fn clone(&self) -> Self {
        Transform {
            pos: self.pos.clone(),
            scale: self.scale.clone(),
            vel: self.vel.clone(),
            acc: self.acc.clone(),
            rot: self.rot.clone(),
            rot_vel: self.rot_vel.clone(),
            rot_acc: self.rot_acc.clone(),
            frozen: self.frozen.clone(),
        }
    }
}

