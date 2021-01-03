#![feature(const_generics)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod world;
mod util;
mod state;
mod backend;

extern crate winit;
extern crate wgpu;
extern crate futures;
#[macro_use] extern crate log;
#[macro_use] extern crate downcast_rs;
extern crate rand;
extern crate rand_core;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};
use futures::executor::block_on;
use crate::state::State;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    // Since main can't be async, we're going to need to block
    let mut state = block_on(State::new(&window, false));
    state.update_graphics_data();

    let mut window_focused = false;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                let selected = window_id == window.id();

                match event {
                    WindowEvent::CloseRequested if selected => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } if selected => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::E),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => state.set_escape_status(&window, true),
                            _ => {}
                        }
                    },
                    WindowEvent::Resized(physical_size) if selected => {
                        state.resize(*physical_size, &window);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } if selected  => {
                        // new_inner_size is &mut so w have to dereference it twice
                        state.resize(**new_inner_size, &window);
                    },
                    WindowEvent::Focused(focused) => {
                        window_focused = *focused;
                        state.set_escape_status(&window, !window_focused);
                    },
                    WindowEvent::CursorEntered {..} => {
                        state.set_escape_status(&window, false);
                    },
                    WindowEvent::CursorLeft {..} => {
                        state.set_escape_status(&window, true);
                    },
                    WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, ..} => {
                        state.set_escape_status(&window, false);
                    },
                    _ => {}
                }
            },
            Event::RedrawRequested(_) => {
                state.update();
                state.render();
            },
            Event::DeviceEvent{event, ..} if window_focused => {
                state.input(&event, &window);
            },
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn handle_window_selection_change(window: &Window, focused: bool) {

}