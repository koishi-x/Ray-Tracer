#![allow(unused)]

use image::GenericImageView;

use crate::*;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        //Image 11
        // Color {
        //     x: 1.0,
        //     y: 1.0,
        //     z: 1.0,
        // } * 0.5
        //     * (1.0 + self.noise.noise(p * self.scale))

        //Image 12
        // Color {
        //     x: 1.0,
        //     y: 1.0,
        //     z: 1.0,
        // } * self.noise.turb(p * self.scale, 7)

        //Image 13
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        } * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

pub struct ImageTexture {
    data: image::DynamicImage,
    width: u32,
    height: u32,
    //bytes_per_scanline: i32,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let data = image::open(filename).expect("Cannot open.");
        let (width, height) = image::GenericImageView::dimensions(&data);
        ImageTexture {
            data,
            width,
            height,
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, p: Point3) -> Color {
        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel = self.data.get_pixel(i, j);
        Color {
            x: pixel[0] as f64 * color_scale,
            y: pixel[1] as f64 * color_scale,
            z: pixel[2] as f64 * color_scale,
        }
    }
}
