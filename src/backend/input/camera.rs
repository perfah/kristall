use std::time::Duration;
use winit::window::Window;
use winit::event::{DeviceEvent, ElementState, VirtualKeyCode, KeyboardInput, MouseScrollDelta};
use winit::dpi::{Position, LogicalPosition, PhysicalSize};
use crate::backend::graphics::camera::CameraPerspective;

pub trait CameraController: Sync + Send {
    fn on_resize(&mut self, window: &Window);
    fn on_escape_status_change(&mut self, window: &Window, escape_status: bool);
    fn on_update_perspective(&mut self, perspective: &mut CameraPerspective, delta: Duration);
    fn on_incoming_event(&mut self, incoming_event: &DeviceEvent, window: &Window) -> bool;
}

pub struct KeyArrowCameraController {
    speed: f32,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    zoom_in: bool,
    zoom_out: bool
}

impl KeyArrowCameraController {
    pub fn new() -> KeyArrowCameraController{
        KeyArrowCameraController { 
            speed: 0.001f32,
            left: false,
            right: false,
            up: false,
            down: false,
            zoom_in: false,
            zoom_out: false
        }
    }
}

impl CameraController for KeyArrowCameraController {
    fn on_resize(&mut self, _: &Window) {}

    fn on_escape_status_change(&mut self, _: &Window, _: bool){}

    fn on_update_perspective(&mut self, perspective: &mut CameraPerspective, delta: Duration){
        if self.left {
            match perspective {
                CameraPerspective::ThirdPersonView {ref mut angle_horiz, .. } => *angle_horiz -= self.speed * delta.as_millis() as f32,
                _ => {}
            }
        }
        else if self.right {
            match perspective {
                CameraPerspective::ThirdPersonView {ref mut angle_horiz, .. } => *angle_horiz += self.speed * delta.as_millis() as f32,
                _ => {}
            }
        }

        if self.up {
            match perspective {
                CameraPerspective::ThirdPersonView {ref mut angle_vert, .. } => *angle_vert -= self.speed * delta.as_millis() as f32,
                _ => {}
            }
        }
        else if self.down {
            match perspective {
                CameraPerspective::ThirdPersonView {ref mut angle_vert, .. } => *angle_vert += self.speed * delta.as_millis() as f32,
                _ => {}
            } 
        }

        if self.zoom_in {
            match perspective {
                CameraPerspective::ThirdPersonView {ref mut distance, .. } => *distance += self.speed * delta.as_millis() as f32,
                _ => {}
            }
        }
        else if self.zoom_out {
            match perspective {
                CameraPerspective::ThirdPersonView {ref mut distance, .. } => *distance -= self.speed * delta.as_millis() as f32,
                _ => {}
            }
        }
    }

    fn on_incoming_event(&mut self, incoming_event: &DeviceEvent, _: &Window) -> bool{
        match incoming_event {
            DeviceEvent::Key (KeyboardInput {
                state,
                virtual_keycode: Some(keycode),
                ..
            }) => {
                let pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space => { self.zoom_in = pressed; true }
                    VirtualKeyCode::LShift => { self.zoom_out = pressed; true }
                    VirtualKeyCode::Up => { self.up = pressed; true }
                    VirtualKeyCode::Down => { self.down = pressed; true }
                    VirtualKeyCode::Left => { self.left = pressed; true }
                    VirtualKeyCode::Right => { self.right = pressed; true }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}


pub struct MouseCameraController {
    // Config:
    mouse_sensitivity: f64,
    scroll_sensitivity: f64,
    fast_scroll: bool,

    // Internal parameters:
    midpoint: (f64, f64),
    current_movement: (f64, f64),
    current_scroll_vel: f64,
    mouse_locking: bool,
    closest_zoom: f32
}

impl MouseCameraController {
    pub fn new(mouse_sensitivity: f64, fast_scroll: bool) -> MouseCameraController{
        MouseCameraController { 
            mouse_sensitivity,
            scroll_sensitivity: 0.001f64,
            midpoint: (0f64, 0f64),
            current_movement: (0f64, 0f64),
            current_scroll_vel: 0f64,
            mouse_locking: false,
            fast_scroll,
            closest_zoom: 5f32
        }
    }
}

impl CameraController for MouseCameraController {
    fn on_resize(&mut self, window: &Window) {
        let PhysicalSize::<u32> { width, height } = window.inner_size();
        self.midpoint = (width as f64 / 2f64, height as f64 / 2f64);
    }

    fn on_escape_status_change(&mut self, window: &Window, escape_status: bool){
        self.mouse_locking = !escape_status;
        window.set_cursor_visible(escape_status);
        if let Err(_) = window.set_cursor_grab(!escape_status) {
            // TODO: Handle error
        }
    }
    
    fn on_update_perspective(&mut self, perspective: &mut CameraPerspective, delta: Duration){ 
        let (diff_x, diff_y) = self.current_movement;

        match perspective {
            CameraPerspective::ThirdPersonView {ref mut angle_horiz, ref mut angle_vert, ref mut distance, .. } => {
                // Mouse movement:
                *angle_horiz += (diff_x * self.mouse_sensitivity * delta.as_millis() as f64) as f32;
                *angle_vert -= (diff_y  * self.mouse_sensitivity * delta.as_millis() as f64) as f32;

                // :Scoll:
                if f64::abs(self.current_scroll_vel) > 1f64 { 
                    let scroll_dist = self.current_scroll_vel * delta.as_millis() as f64;
                                
                    *distance += (scroll_dist * self.scroll_sensitivity * delta.as_millis() as f64) as f32;
                    if *distance < self.closest_zoom {
                        *distance = self.closest_zoom;
                    }

                    self.current_scroll_vel -= self.current_scroll_vel  * 
                                               if self.fast_scroll { 0.01f64 } else { 0.05f64 } * 
                                               delta.as_millis() as f64;
                }
            
            },
            _ => {}
        }  

        self.current_movement = (0f64, 0f64);
    }

    fn on_incoming_event(&mut self, incoming_event: &DeviceEvent, window: &Window) -> bool{
        match incoming_event {
            DeviceEvent::MouseMotion { delta: (x, y), } => {
                let (mid_x, mid_y) = self.midpoint;
                self.current_movement = (*x, *y);

                if self.mouse_locking {
                    if let Err(_) = window.set_cursor_position(
                        Position::Logical(LogicalPosition::<f64> { x: mid_x, y: mid_y})){
                        // TODO: Handle error
                    }
                }
                true
            },
            DeviceEvent::MouseWheel { delta } => {
                self.current_scroll_vel += match delta {
                    MouseScrollDelta::LineDelta(_, vert) => { *vert as f64 },
                    MouseScrollDelta::PixelDelta(pos) => { pos.y }
                };

                true
            }
            _ => false,
        }
    }
}
