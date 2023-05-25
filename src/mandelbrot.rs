use rayon::prelude::*;
use slice_of_array::prelude::*;

use num::{complex::ComplexFloat, Complex};
use std::time::Instant;

use crate::color;

const MAX_ITER: i32 = 50;

pub const MIN_X: f32 = -2.2;
pub const MAX_X: f32 = 0.8;
pub const MIN_Y: f32 = -1.125;
pub const MAX_Y: f32 = 1.125;

pub fn get_title() -> String {
    format!(
        "Mandelbrot - x ∈ [{},{}] - y ∈ [{},{}]",
        MIN_X, MAX_X, MIN_Y, MAX_Y
    )
}

pub fn draw_mandelbrot(frame: &mut [u8], width: u32, height: u32) {
    let start = Instant::now();

    let pixels: Vec<_> = (0..(frame.len() as u32 / 4))
        .into_par_iter()
        .map(|i| {
            let x = (i % width) as f32 / width as f32;
            let y = (i / width) as f32 / height as f32;

            let x = MIN_X + x * (MAX_X - MIN_X);
            let y = MAX_Y - y * (MAX_Y - MIN_Y);

            let res = calc_mandelbrot(x, y);
            color::to_color(res)
        })
        .collect();

    frame.copy_from_slice(pixels.flat());

    let pixel_count = pixels.len();
    let total_time = start.elapsed().as_secs_f32() * 1000.0;
    println!("Mandelbrot: {} px, {:.2} ms", pixel_count, total_time,);
}

fn calc_mandelbrot(x: f32, y: f32) -> f32 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for n in 1..=MAX_ITER {
        z = z * z + c;

        if z.abs() > 1000.0 {
            return n as f32 / MAX_ITER as f32;
        }
    }

    0.0
}
