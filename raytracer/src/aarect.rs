#![allow(dead_code)]

use crate::*;

pub struct XYRect {
    pub mp: Arc<dyn Material + Send + Sync>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl XYRect {
    pub fn new(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> XYRect {
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

pub struct XZRect {
    pub mp: Arc<dyn Material + Send + Sync>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl XZRect {
    pub fn new(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> XZRect {
        XZRect {
            mp: mat,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: Point3 {
                x: self.x0,
                y: self.k - 0.0001,
                z: self.z0,
            },
            maximum: Point3 {
                x: self.x1,
                y: self.k + 0.0001,
                z: self.z1,
            },
        })
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + r.dir.x * t;
        let z: f64 = r.orig.z + r.dir.z * t;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new(t, r.at(t), &self.mp);
        let outward_normal = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        rec.set_face_normal(r, outward_normal);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        Some(rec)
    }
}

pub struct YZRect {
    pub mp: Arc<dyn Material + Send + Sync>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl YZRect {
    pub fn new(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> YZRect {
        YZRect {
            mp: mat,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB {
            minimum: Point3 {
                x: self.k - 0.0001,
                y: self.y0,
                z: self.z0,
            },
            maximum: Point3 {
                x: self.k + 0.0001,
                y: self.y1,
                z: self.z1,
            },
        })
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.y + r.dir.y * t;
        let z: f64 = r.orig.z + r.dir.z * t;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new(t, r.at(t), &self.mp);
        let outward_normal = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        rec.set_face_normal(r, outward_normal);
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        Some(rec)
    }
}
