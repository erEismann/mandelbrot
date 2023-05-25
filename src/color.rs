const MAX_RED: f32 = 225.0;
const MAX_BLUE: f32 = 245.0;
const MAX_GREEN: f32 = 255.0;

const FIRST_SEG: f32 = 0.25;
const SECOND_SEG: f32 = 0.1;
const THIRD_SEG: f32 = 1.0 - FIRST_SEG - SECOND_SEG;

pub fn to_color(n: f32) -> [u8; 4] {
    if n == 0.0 {
        return [0, 0, 0, 0];
    }

    let r;
    let g;
    let b;

    // red to blue
    if n <= FIRST_SEG {
        r = MAX_RED * (1.0 - (1.0 / FIRST_SEG) * n);
        g = MAX_GREEN * (0.3 / FIRST_SEG) * n;
        b = MAX_BLUE * (1.0 / FIRST_SEG) * n;

    // blue to green
    } else if n <= FIRST_SEG + SECOND_SEG {
        let n = n - FIRST_SEG;
        r = 0.0;
        g = MAX_GREEN * (0.3 + (0.7 / SECOND_SEG) * n);
        b = MAX_BLUE * (1.0 - (1.0 / SECOND_SEG) * n);

    // green tu purple
    } else {
        let n = n - FIRST_SEG - SECOND_SEG;
        r = MAX_RED * (0.7 / THIRD_SEG) * n;
        g = MAX_GREEN * (1.0 - (1.0 / THIRD_SEG) * n);
        b = MAX_BLUE * (1.0 / THIRD_SEG) * n;
    }

    [r as u8, g as u8, b as u8, 255]
}
