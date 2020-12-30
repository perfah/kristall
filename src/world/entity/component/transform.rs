use std::sync::Arc;
use crate::world::entity::component::{Component};
use crate::backend::graphics::transform::{TransformSink, ModelView};
use failure::_core::any::Any;
use cgmath::{Vector3, Quaternion};
use crate::backend::BackendProxy;

pub struct Transform {
    pub pos: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub acc: Vector3<f32>,
    pub rot: Quaternion<f32>,
    pub rot_vel: Quaternion<f32>,
    pub rot_acc: Quaternion<f32>,
    pub sink: Arc<TransformSink>

}

impl Transform {
    pub fn new(backend_proxy: &BackendProxy) -> Transform {
        Transform {
            pos: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            scale: Vector3 {x: 1.0, y: 1.0, z: 1.0},
            vel: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            acc: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            rot: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            rot_vel: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            rot_acc: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            sink: Arc::new(backend_proxy.new_transform_sink())
        }
    }

    pub fn with_position(mut self, pos: Vector3<f32>) -> Self{
        self.pos = pos;
        self.sink.update(self.to_raw());
        self
    }

    pub fn with_velocity(mut self, vel: Vector3<f32>) -> Self{
        self.vel = vel;
        self.sink.update(self.to_raw());
        self
    }

    pub fn flush(&self) {
        self.sink.update(self.to_raw());
    }
}

impl Transform {
    pub fn to_raw(&self) -> ModelView {
        ModelView {
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

