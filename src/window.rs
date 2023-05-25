use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    window::{Window, WindowBuilder, WindowId},
};

pub struct MyWindow<F>
where
    F: FnMut(&mut [u8], u32, u32),
{
    pub id: WindowId,
    window: Window,
    pixels: Pixels,
    render: F,
}

impl<F> MyWindow<F>
where
    F: FnMut(&mut [u8], u32, u32),
{
    pub fn new(
        event_loop: &EventLoop<()>,
        width: u32,
        height: u32,
        title: String,
        render: F,
    ) -> Self {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        let tex = SurfaceTexture::new(width, height, &window);
        let pixels = Pixels::new(width, height, tex).unwrap();

        let id = window.id();
        MyWindow {
            id,
            window,
            pixels,
            render,
        }
    }

    pub fn redraw(&mut self) {
        let PhysicalSize { width, height } = self.window.inner_size();
        if width != 0 && height != 0 {
            (self.render)(self.pixels.frame_mut(), width, height);
            self.pixels.render().unwrap();
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let PhysicalSize { width, height } = size;
        if width != 0 && height != 0 {
            self.pixels.resize_surface(width, height).unwrap();
            self.pixels.resize_buffer(width, height).unwrap();
            self.window.request_redraw();
        }
    }

    pub fn hover(&mut self, pos: PhysicalPosition<f64>) -> (f32, f32) {
        let PhysicalPosition { x, y } = pos;
        let PhysicalSize { width, height } = self.window.inner_size();
        let x = x as f32 / width as f32;
        let y = y as f32 / height as f32;
        (x, y)
    }
}
