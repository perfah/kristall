
use crate::backend::graphics::OPENGL_TO_WGPU_MATRIX;
use winit::event::{WindowEvent, ElementState, VirtualKeyCode, KeyboardInput};
use cgmath::{Point3, Vector3, EuclideanSpace, InnerSpace};
use crate::world::entity::component::{Component, ComponentManager};
use crate::world::entity::component::transform::Transform;

pub use crate::backend::graphics::camera::CameraPerspective;
pub use crate::backend::input::camera::CameraController;

pub struct Camera {
    pub perspective: CameraPerspective,
    pub controller: Box<dyn CameraController>
}

impl Camera {
    pub fn new<T: 'static + CameraController>(perspective: CameraPerspective, controller: T) -> Camera {
        Camera { 
            perspective, 
            controller: Box::new(controller)
        } 
    }
}

impl Component for Camera {
    fn enabled(&self) -> bool {
        unimplemented!()
    }
}


