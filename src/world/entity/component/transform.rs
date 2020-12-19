use crate::world::entity::component::{Component};
use crate::util::wgpu::uniform::{TransformRaw};
use failure::_core::any::Any;
use cgmath::{Vector3, Quaternion};

pub struct Transform {
    pub pos: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub acc: Vector3<f32>,

    pub rot: Quaternion<f32>,
    pub rot_vel: Quaternion<f32>,
    pub rot_acc: Quaternion<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            pos: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            scale: Vector3 {x: 1.0, y: 1.0, z: 1.0},
            vel: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            acc: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            rot: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            rot_vel: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            rot_acc: Quaternion::new(0.0, 0.0, 0.0, 0.0)
        }
    }

    pub fn with_position(mut self, pos: Vector3<f32>) -> Self{
        self.pos = pos;
        self
    }

    pub fn with_velocity(mut self, vel: Vector3<f32>) -> Self{
        self.vel = vel;
        self
    }
}

impl Transform {
    pub fn to_raw(&self) -> TransformRaw {
        TransformRaw {
            model: (
                cgmath::Matrix4::from_translation(self.pos) *
                cgmath::Matrix4::from(self.rot)
            ).into(),
        }
    }
}

impl Component for Transform {
    fn enabled(&self) -> bool {
        true
    }
}

impl Clone for Transform{
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

