use window::{JLMbValues, JlMbMode, JlMbWindow};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
};

mod color;
mod window;

fn main() {
    let event_loop = EventLoop::new();

    let vals = JLMbValues::new(-2.2, 0.8, -1.125, 1.125, JlMbMode::new_mb());
    let mut mb = JlMbWindow::new(&event_loop, 1200, 800, "Mandelbrot", vals);

    let vals = JLMbValues::new(-2.0, 2.0, -2.0, 2.0, JlMbMode::new_jl());
    let mut jl = JlMbWindow::new(&event_loop, 800, 800, "Julia", vals);

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
                    let val = mb.get_hover_pos(position);
                    jl.set_init_val(val);
                    jl.redraw();
                }
            }

            _ => (),
        }
    });
}
