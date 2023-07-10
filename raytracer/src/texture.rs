#![allow(unused)]

use crate::*;

pub trait Texture {
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
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            odd: Rc::new(SolidColor::new(c1)),
            even: Rc::new(SolidColor::new(c2)),
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
        Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        } * 0.5
            * (1.0 + self.noise.noise(p * self.scale))
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
