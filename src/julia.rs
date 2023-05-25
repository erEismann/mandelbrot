use rayon::prelude::*;
use slice_of_array::prelude::*;

use num::{complex::ComplexFloat, Complex};
use std::time::Instant;

use crate::color;

const MAX_ITER: i32 = 50;

const MIN_X: f32 = -2.0;
const MAX_X: f32 = 2.0;
const MIN_Y: f32 = -2.0;
const MAX_Y: f32 = 2.0;

pub fn get_title() -> String {
    format!(
        "Julia - x ∈ [{},{}] - y ∈ [{},{}]",
        MIN_X, MAX_X, MIN_Y, MAX_Y
    )
}

pub fn draw_julia(frame: &mut [u8], width: u32, height: u32, c_x: f32, c_y: f32) {
    let start = Instant::now();

    let pixels: Vec<_> = (0..(frame.len() as u32 / 4))
        .into_par_iter()
        .map(|i| {
            let x = (i % width) as f32 / width as f32;
            let y = (i / width) as f32 / height as f32;

            let x = MIN_X + x * (MAX_X - MIN_X);
            let y = MAX_Y - y * (MAX_Y - MIN_Y);

            let res = calc_julia(x, y, c_x, c_y);
            color::to_color(res)
        })
        .collect();

    frame.copy_from_slice(pixels.flat());

    let pixel_count = pixels.len();
    let total_time = start.elapsed().as_secs_f32() * 1000.0;
    println!("Julia: {} px, {:.2} ms", pixel_count, total_time,);
}

fn calc_julia(x: f32, y: f32, c_x: f32, c_y: f32) -> f32 {
    let c = Complex::new(c_x, c_y);
    let mut z = Complex::new(x, y);

    for n in 1..=MAX_ITER {
        z = z * z + c;

        if z.abs() > 1000.0 {
            return n as f32 / MAX_ITER as f32;
        }
    }

    0.0
}
