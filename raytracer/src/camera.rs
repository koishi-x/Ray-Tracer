use crate::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        // Camera {
        //     origin: Vec3::new(),
        //     lower_left_corner: Vec3::new(),
        //     horizontal: Vec3::new(),
        //     vertical: Vec3::new(),
        // }
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let horizontal = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}
