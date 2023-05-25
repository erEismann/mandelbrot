use std::cell::RefCell;

use julia::draw_julia;
use mandelbrot::{draw_mandelbrot, MAX_X, MAX_Y, MIN_X, MIN_Y};
use window::MyWindow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
};

mod color;
mod julia;
mod mandelbrot;
mod window;

thread_local!(static JL_X: RefCell<f32> = RefCell::default());
thread_local!(static JL_Y: RefCell<f32> = RefCell::default());

fn main() {
    let event_loop = EventLoop::new();

    let title = mandelbrot::get_title();
    let mut mb = MyWindow::new(&event_loop, 1200, 800, title, draw_mandelbrot);

    let title = julia::get_title();
    let mut jl = MyWindow::new(&event_loop, 800, 800, title, |frame, x, y| {
        let j_x = JL_X.with(|jl_x| *jl_x.borrow());
        let j_y = JL_Y.with(|jl_y| *jl_y.borrow());
        draw_julia(frame, x, y, j_x, j_y);
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(window_id) => {
                if window_id == mb.id {
                    mb.redraw();
                } else if window_id == jl.id {
                    jl.redraw();
                }
            }

            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
            } => {
                if window_id == mb.id {
                    mb.resize(size);
                } else if window_id == jl.id {
                    jl.resize(size);
                }
            }

            Event::WindowEvent {
                window_id,
                event: WindowEvent::CursorMoved { position, .. },
            } => {
                if window_id == mb.id {
                    let (x, y) = mb.hover(position);

                    let x = MIN_X + x * (MAX_X - MIN_X);
                    let y = MAX_Y - y * (MAX_Y - MIN_Y);

                    JL_X.with(|jl_x| *jl_x.borrow_mut() = x);
                    JL_Y.with(|jl_y| *jl_y.borrow_mut() = y);

                    jl.redraw();
                }
            }

            _ => (),
        }
    });
}
