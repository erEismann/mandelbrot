use rayon::prelude::*;
use std::time::Instant;

use num::{complex::ComplexFloat, Complex};

const MAX_ITER: i32 = 50;

const MANDELBROT_MIN_X: f32 = -2.2;
const MANDELBROT_MAX_X: f32 = 0.8;
const MANDELBROT_MIN_Y: f32 = -1.125;
const MANDELBROT_MAX_Y: f32 = 1.125;

pub fn draw_mandelbrot(mut frame: &mut [u8], width: u32, height: u32) {
    let start = Instant::now();

    let hues: Vec<f32> = (0..(frame.len() as u32 / 4))
        .into_par_iter()
        .map(|i| {
            let x = (i % width) as f32 / width as f32;
            let y = (i / width) as f32 / height as f32;

            let x = MANDELBROT_MIN_X + x * (MANDELBROT_MAX_X - MANDELBROT_MIN_X);
            let y = MANDELBROT_MAX_Y - y * (MANDELBROT_MAX_Y - MANDELBROT_MIN_Y);

            calc_mandelbrot(x, y)
        })
        .collect();

    let pixel_count = hues.len();

    println!("{}", start.elapsed().as_secs_f32() * 1000.0);

    for hue in hues {
        if hue > 0.0 {
            let (r, g, b) = hue_to_rgb(hue);
            (frame[0], frame[1], frame[2], frame[3]) = (r as u8, g as u8, b as u8, 255);
        } else {
            (frame[0], frame[1], frame[2], frame[3]) = (0, 0, 0, 0);
        }
        frame = &mut frame[4..];
    }

    println!(
        "calulated {} pixels, took {} millis",
        pixel_count,
        start.elapsed().as_secs_f32() * 1000.0
    );
}

fn calc_mandelbrot(x: f32, y: f32) -> f32 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for n in 0..MAX_ITER {
        z = z * z + c;

        if z.abs() > 1000.0 {
            return 360.0 * (1.0 - (n - 6) as f32 / MAX_ITER as f32);
        }
    }

    0.0
}

fn hue_to_rgb(mut h: f32) -> (u8, u8, u8) {
    if h >= 360.0 {
        h -= 360.0;
    } else if h < 0.0 {
        h += 360.0;
    }

    let hh = h / 60.0;
    let i = hh as u32;
    let ff = hh - i as f32;

    let t = (ff * 255.0) as u8;
    let q = 255 - t;

    match i {
        0 => (255, t, 0),
        1 => (q, 255, 0),
        2 => (0, 255, t),
        3 => (0, q, 255),
        4 => (t, 0, 255),
        5 => (255, 0, q),
        _ => panic!("invalid i {} {}", i, h),
    }
}