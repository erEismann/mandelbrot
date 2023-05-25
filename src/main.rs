use std::collections::BTreeMap;

use mandelbrot::draw_mandelbrot;
use window::MyWindow;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
};

mod mandelbrot;
mod window;

fn main() {
    let event_loop = EventLoop::new();
    let mut windows = BTreeMap::new();

    let (id, mb) = MyWindow::new(&event_loop, draw_mandelbrot);
    windows.insert(id, mb);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(window_id) => {
                if let Some(window) = windows.get_mut(&window_id) {
                    window.redraw();
                }
            }

            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
            } => {
                if let Some(window) = windows.get_mut(&window_id) {
                    let PhysicalSize { width, height } = size;
                    window.resize(width, height)
                }
            }

            _ => (),
        }
    });
}
