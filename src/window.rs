use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoop,
    window::{Window, WindowBuilder, WindowId},
};

const INIT_WIDTH: u32 = 1200;
const INIT_HEIGHT: u32 = 800;

pub struct MyWindow {
    window: Window,
    pixels: Pixels,
    render: fn(&mut [u8], u32, u32),
}

impl MyWindow {
    pub fn new(event_loop: &EventLoop<()>, render: fn(&mut [u8], u32, u32)) -> (WindowId, Self) {
        let window = WindowBuilder::new()
            .with_title("Mandelbrot")
            .with_inner_size(LogicalSize::new(INIT_WIDTH, INIT_HEIGHT))
            .build(&event_loop)
            .unwrap();

        let tex = SurfaceTexture::new(INIT_WIDTH, INIT_HEIGHT, &window);
        let pixels = Pixels::new(INIT_WIDTH, INIT_HEIGHT, tex).unwrap();

        let id = window.id();

        let mb = MyWindow {
            window,
            pixels,
            render,
        };

        (id, mb)
    }

    pub fn redraw(&mut self) {
        let PhysicalSize { width, height } = self.window.inner_size();
        (self.render)(self.pixels.frame_mut(), width, height);
        self.pixels.render().unwrap();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height).unwrap();
        self.pixels.resize_buffer(width, height).unwrap();
        self.window.request_redraw();
    }
}
