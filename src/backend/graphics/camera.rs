use std::time::Duration;

use crate::backend::graphics::OPENGL_TO_WGPU_MATRIX;
use winit::window::Window;
use winit::event::{DeviceEvent, ElementState, VirtualKeyCode, KeyboardInput};
use cgmath::{Point3, Vector3, EuclideanSpace, InnerSpace};
use crate::world::entity::component::{Component, ComponentManager};
use crate::world::entity::component::transform::Transform;
use crate::backend::input::camera::CameraController;

use crate::world::entity::component::camera::Camera as CameraComponent;

pub enum CameraPerspective {
    FirstPersonView,
    ThirdPersonView {
        distance: f32,
        angle_horiz: f32,
        angle_vert: f32
    },
    BirdsEyeView
}

impl CameraPerspective {
    pub fn new_third_person() -> CameraPerspective {
        CameraPerspective::ThirdPersonView {
            distance: 25f32,
            angle_horiz: 0f32,
            angle_vert: 0f32
        }
    }
}


pub struct Camera {
    component: ComponentManager<CameraComponent>,
    target: ComponentManager<Transform>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(component: ComponentManager<CameraComponent>, 
               target: ComponentManager<Transform>) -> Camera {
        Camera {
            component,
            target,
            aspect: 10 as f32 / 10 as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }


    fn get_view(&self) -> cgmath::Matrix4<f32> {
        let target = self.target.peek(|transform| Point3::from_vec(transform.pos)).unwrap();
        let perspective = &self.component.lock_component_for_read().perspective;

        match perspective {
            CameraPerspective::FirstPersonView => unimplemented!(),
            CameraPerspective::ThirdPersonView { distance, angle_horiz, angle_vert } => {
                let C = 2.0 * std::f32::consts::PI;
        
                let dir = Vector3::new(
                    f32::cos(angle_horiz % C) * f32::cos(angle_vert % C),
                    f32::sin(angle_vert % C),
                    f32::sin(angle_horiz % C) * f32::cos(angle_vert % C)
                ).normalize();

                let eye = target - dir * (*distance);
                
                cgmath::Matrix4::look_at(
                    eye, 
                    target, 
                    cgmath::Vector3::unit_y() * f32::signum(f32::cos(angle_vert % C)))
            },
            CameraPerspective::BirdsEyeView => unimplemented!(),
            _ => unimplemented!()
        }
    }

    pub fn view_proj_matrix(&self) -> cgmath::Matrix4<f32> {
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let view = self.get_view();
        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn process_events(&mut self, event: &DeviceEvent, window: &Window) -> bool {
        let controller = &mut self.component.lock_component_for_write().controller;
        controller.on_incoming_event(event, window)
    }
    
    pub fn update(&mut self, delta: Duration) {
        let CameraComponent { ref mut perspective, ref mut controller } = *self.component.lock_component_for_write();
        controller.on_update_perspective(perspective, delta);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, window: &Window) {
        let controller = &mut self.component.lock_component_for_write().controller;

        self.aspect = new_size.width as f32 / new_size.height as f32;
        controller.on_resize(window);
    }

    pub fn set_escape_status(&mut self, window: &Window, escape_status: bool) {
        let controller = &mut self.component.lock_component_for_write().controller;
        controller.on_escape_status_change(window, escape_status)
    }
}
