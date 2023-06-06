use pixels::{Pixels, SurfaceTexture};
use rayon::prelude::*;
use slice_of_array::prelude::*;
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    window::{Window, WindowBuilder, WindowId},
};

use num::{complex::ComplexFloat, Complex, Zero};
use std::time::Instant;

use crate::color;

const MAX_ITER: i32 = 50;

pub struct JlMbWindow {
    pub id: WindowId,
    name: &'static str,
    window: Window,
    pixels: Pixels,
    vals: JLMbValues,
}

pub struct JLMbValues {
    min: Complex<f32>,
    max: Complex<f32>,
    mode: JlMbMode,
}

pub enum JlMbMode {
    Julia(Complex<f32>),
    Mandelbrot,
}

impl JlMbWindow {
    pub fn new(
        event_loop: &EventLoop<()>,
        width: u32,
        height: u32,
        name: &'static str,
        values: JLMbValues,
    ) -> Self {
        let title = format!(
            "{} - Re ∈ [{},{}] - Im ∈ [{},{}]",
            name, values.min.re, values.max.re, values.min.im, values.max.im,
        );

        let size = LogicalSize::new(width, height);

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap();

        let tex = SurfaceTexture::new(width, height, &window);
        let pixels = Pixels::new(width, height, tex).unwrap();
        let id = window.id();

        JlMbWindow {
            id,
            name,
            window,
            pixels,
            vals: values,
        }
    }

    pub fn redraw(&mut self) {
        self.draw();
        self.pixels.render().unwrap();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let PhysicalSize { width, height } = size;
        if width != 0 && height != 0 {
            self.pixels.resize_surface(width, height).unwrap();
            self.pixels.resize_buffer(width, height).unwrap();
            self.window.request_redraw();
        }
    }

    pub fn get_hover_pos(&mut self, pos: PhysicalPosition<f64>) -> Complex<f32> {
        let PhysicalPosition { x, y } = pos;
        let PhysicalSize { width, height } = self.window.inner_size();

        let x = x as f32 / width as f32;
        let y = y as f32 / height as f32;

        let re = self.vals.min.re + x * (self.vals.max.re - self.vals.min.re);
        let im = self.vals.max.im - y * (self.vals.max.im - self.vals.min.im);

        Complex::new(re, im)
    }

    pub fn set_init_val(&mut self, init: Complex<f32>) {
        if let JlMbMode::Julia(jl_init) = &mut self.vals.mode {
            *jl_init = init;
        }
    }

    fn draw(&mut self) {
        let PhysicalSize { width, height } = self.window.inner_size();
        if width == 0 || height == 0 {
            return;
        }

        let start = Instant::now();

        let pixel_count = self.pixels.frame().len() as u32 / 4;
        let pixels: Vec<_> = (0..pixel_count)
            .into_par_iter()
            .map(|i| {
                let x = (i % width) as f32 / width as f32;
                let y = (i / width) as f32 / height as f32;

                let re = self.vals.min.re + x * (self.vals.max.re - self.vals.min.re);
                let im = self.vals.max.im - y * (self.vals.max.im - self.vals.min.im);

                let c = Complex::new(re, im);

                let res = match self.vals.mode {
                    JlMbMode::Mandelbrot => self.calc(Complex::zero(), c),
                    JlMbMode::Julia(z) => self.calc(c, z),
                };
                color::to_color(res)
            })
            .collect();

        self.pixels.frame_mut().copy_from_slice(pixels.flat());

        let pixel_count = pixels.len();
        let total_time = start.elapsed().as_secs_f32() * 1000.0;

        println!("{}: {} px, {:.2} ms", self.name, pixel_count, total_time,);
    }

    fn calc(&self, mut z: Complex<f32>, c: Complex<f32>) -> f32 {
        for n in 1..=MAX_ITER {
            z = z * z + c;

            if z.abs() > 1000.0 {
                return n as f32 / MAX_ITER as f32;
            }
        }

        0.0
    }
}

impl JLMbValues {
    pub fn new(min_re: f32, max_re: f32, min_im: f32, max_im: f32, mode: JlMbMode) -> Self {
        let min = Complex::new(min_re, min_im);
        let max = Complex::new(max_re, max_im);
        JLMbValues { min, max, mode }
    }
}

impl JlMbMode {
    pub fn new_mb() -> Self {
        Self::Mandelbrot
    }

    pub fn new_jl() -> Self {
        Self::Julia(Complex::zero())
    }
}
