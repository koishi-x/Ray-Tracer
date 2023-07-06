use crate::{clamp, Vec3};

pub fn write_color(pixel_color: Vec3, samples_per_pixel: u32) -> [u8; 3] {
    let scale = 1.0 / (samples_per_pixel as f64);
    [
        (clamp(pixel_color.x * scale, 0.0, 0.999) * 256.0) as u8,
        (clamp(pixel_color.y * scale, 0.0, 0.999) * 256.0) as u8,
        (clamp(pixel_color.z * scale, 0.0, 0.999) * 256.0) as u8,
    ]
}
