#![allow(dead_code)]

use crate::*;

pub struct XYRect {
    pub mp: Rc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Rc<dyn Material>) -> XYRect {
        XYRect {
            mp: mat,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: Point3 {
                x: self.x0,
                y: self.y0,
                z: self.k - 0.0001,
            },
            maximum: Point3 {
                x: self.x1,
                y: self.y1,
                z: self.k + 0.0001,
            },
        })
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + r.dir.x * t;
        let y = r.orig.y + r.dir.y * t;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord::new(t, r.at(t), &self.mp);
        let outward_normal = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        rec.set_face_normal(r, outward_normal);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        Some(rec)
    }
}
