use crate::*;

pub fn write_color(pixel_color: Vec3, samples_per_pixel: u32) -> [u8; 3] {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }
    let scale = 1.0 / (samples_per_pixel as f64);
    [
        (clamp((r * scale).sqrt(), 0.0, 0.999) * 256.0) as u8,
        (clamp((g * scale).sqrt(), 0.0, 0.999) * 256.0) as u8,
        (clamp((b * scale).sqrt(), 0.0, 0.999) * 256.0) as u8,
    ]
}

pub struct ColorInformation {
    pub i: u32,
    pub j: u32,
    pub color: Color,
}

impl ColorInformation {
    pub fn new(i: u32, j: u32, color: Color) -> ColorInformation {
        ColorInformation { i, j, color }
    }
}
