use std::sync::Arc;
use crate::world::entity::component::{Component};
use crate::backend::graphics::model_view::{ModelView};
use failure::_core::any::Any;
use cgmath::{Vector3, Quaternion};
use crate::backend::BackendProxy;

pub struct Transform {
    pub position: Vector3<f32>,
    pub angular_rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            angular_rotation: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            scale: Vector3 {x: 1.0, y: 1.0, z: 1.0},
        }
    }

    pub fn with_position(mut self, position: Vector3<f32>) -> Self{
        self.position = position;
        self
    }

    pub fn with_offset(mut self, offset: &Transform) -> Self {
        self.position += offset.position;

        // TODO: Definitely incorrect:
        self.angular_rotation += offset.angular_rotation;

        // TODO: Check if this is the way to do it:
        self.scale.x *= offset.scale.x;
        self.scale.y *= offset.scale.y;
        self.scale.z *= offset.scale.z;

        self
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
            position: self.position.clone(),
            angular_rotation: self.angular_rotation.clone(),
            scale: self.scale.clone(),
        }
    }
}

