pub use console::style;
pub use image::flat::View;
pub use image::{ImageBuffer, RgbImage};
pub use indicatif::ProgressBar;
use rand::Rng;
pub use std::f64::INFINITY;
pub use std::{fs::File, process::exit};

//pub mod

pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double(min: f64, max: f64) -> f64 {
    //rand::thread_rng().gen_range(min..max)
    min + (max - min) * random_double_default()
}

pub fn random_double_default() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
