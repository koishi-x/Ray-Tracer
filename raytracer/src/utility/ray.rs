use crate::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    // pub fn new() -> Self {
    //     Ray {
    //         orig: Vec3::new(),
    //         dir: Vec3::new(),
    //     }
    // }
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            tm: 0.0,
        }
    }

    pub fn new_tm(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        //self.orig.plus(&self.dir.multiply(t))
        self.orig + self.dir * t
    }
}
