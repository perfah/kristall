use crate::world::entity::component::{Component};
use crate::backend::graphics::uniform::{TransformRaw};
use failure::_core::any::Any;
use cgmath::{Vector3, Quaternion};

pub struct Transform {
    pub pos: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub acc: Vector3<f32>,

    pub rot: Vector3<f32>,
    pub rot_vel: Vector3<f32>,
    pub rot_acc: Vector3<f32>,

    pub frozen: bool
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
            frozen: false
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
}

impl Transform {
    pub fn to_raw(&self) -> TransformRaw {
        let c = 2.0 * std::f32::consts::PI;

        TransformRaw {
            model: (
                cgmath::Matrix4::from_translation(self.pos) *
                cgmath::Matrix4::from_angle_x(cgmath::Rad(self.rot.x % c)) *
                cgmath::Matrix4::from_angle_y(cgmath::Rad(self.rot.y % c)) *
                cgmath::Matrix4::from_angle_z(cgmath::Rad(self.rot.z % c))
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

