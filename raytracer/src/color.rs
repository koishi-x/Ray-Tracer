use crate::*;

pub fn write_color(pixel_color: Vec3, samples_per_pixel: u32) -> [u8; 3] {
    let scale = 1.0 / (samples_per_pixel as f64);
    [
        (clamp((pixel_color.x * scale).sqrt(), 0.0, 0.999) * 256.0) as u8,
        (clamp((pixel_color.y * scale).sqrt(), 0.0, 0.999) * 256.0) as u8,
        (clamp((pixel_color.z * scale).sqrt(), 0.0, 0.999) * 256.0) as u8,
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
