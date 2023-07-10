use crate::*;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> AABB {
        AABB {
            minimum: a,
            maximum: b,
        }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> Option<()> {
        let mut inv_d = 1.0 / r.dir.x;
        let mut t0 = (self.minimum.x - r.orig.x) * inv_d;
        let mut t1 = (self.maximum.x - r.orig.x) * inv_d;
        if inv_d < 0.0 {
            t_min = f64::max(t_min, t1);
            t_max = f64::min(t_max, t0);
        } else {
            t_min = f64::max(t_min, t0);
            t_max = f64::min(t_max, t1);
        }
        if t_max <= t_min {
            return None;
        }

        inv_d = 1.0 / r.dir.y;
        t0 = (self.minimum.y - r.orig.y) * inv_d;
        t1 = (self.maximum.y - r.orig.y) * inv_d;
        if inv_d < 0.0 {
            t_min = f64::max(t_min, t1);
            t_max = f64::min(t_max, t0);
        } else {
            t_min = f64::max(t_min, t0);
            t_max = f64::min(t_max, t1);
        }
        if t_max <= t_min {
            return None;
        }

        inv_d = 1.0 / r.dir.z;
        t0 = (self.minimum.z - r.orig.z) * inv_d;
        t1 = (self.maximum.z - r.orig.z) * inv_d;
        if inv_d < 0.0 {
            t_min = f64::max(t_min, t1);
            t_max = f64::min(t_max, t0);
        } else {
            t_min = f64::max(t_min, t0);
            t_max = f64::min(t_max, t1);
        }
        if t_max <= t_min {
            return None;
        }
        Some(())
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3 {
        x: f64::min(box0.minimum.x, box1.minimum.x),
        y: f64::min(box0.minimum.y, box1.minimum.y),
        z: f64::min(box0.minimum.z, box1.minimum.z),
    };
    let big = Point3 {
        x: f64::max(box0.maximum.x, box1.maximum.x),
        y: f64::max(box0.maximum.y, box1.maximum.y),
        z: f64::max(box0.maximum.z, box1.maximum.z),
    };
    AABB::new(small, big)
}
