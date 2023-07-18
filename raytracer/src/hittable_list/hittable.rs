use crate::*;

pub use std::sync::Arc;
pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
    fn pdf_value(&self, _o: Point3, _v: Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    // pub fn new() -> Self {
    //     HitRecord {
    //         p: Vec3::new(),
    //         normal: Vec3::new(),
    //         t: 0.0,
    //         front_face: false,
    //     }
    // }
    pub fn new(t: f64, p: Vec3, mat_ptr: &Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p,
            normal: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            mat_ptr: mat_ptr.clone(),
            t,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct Translate {
    pub ptr: Arc<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Translate {
        Translate {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new_tm(r.orig - self.offset, r.dir, r.tm);
        match self.ptr.hit(&moved_r, t_min, t_max) {
            None => None,
            Some(mut rec) => {
                rec.p += self.offset;
                rec.set_face_normal(&moved_r, rec.normal);
                Some(rec)
            }
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.ptr.bounding_box(time0, time1).map(|output_box| {
            AABB::new(
                output_box.minimum + self.offset,
                output_box.maximum + self.offset,
            )
        })
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box(0.0, 1.0);
        if bbox.is_none() {
            return RotateY {
                ptr: p,
                sin_theta,
                cos_theta,
                bbox,
            };
        }
        let mut min = Point3 {
            x: INFINITY,
            y: INFINITY,
            z: INFINITY,
        };
        let mut max = Point3 {
            x: -INFINITY,
            y: -INFINITY,
            z: -INFINITY,
        };
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.unwrap().maximum.x
                        + (1 - i) as f64 * bbox.unwrap().minimum.x;
                    let y = j as f64 * bbox.unwrap().maximum.y
                        + (1 - j) as f64 * bbox.unwrap().minimum.y;
                    let z = k as f64 * bbox.unwrap().maximum.z
                        + (1 - k) as f64 * bbox.unwrap().minimum.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    min.x = min.x.min(new_x);
                    max.x = max.x.max(new_x);
                    min.y = min.y.min(y);
                    max.y = max.y.max(y);
                    min.z = min.z.min(new_z);
                    max.z = max.z.max(new_z);
                }
            }
        }
        RotateY {
            ptr: p,
            sin_theta,
            cos_theta,
            bbox: Some(AABB::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.bbox
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Point3 {
            x: r.orig.x * self.cos_theta - r.orig.z * self.sin_theta,
            y: r.orig.y,
            z: r.orig.x * self.sin_theta + r.orig.z * self.cos_theta,
        };
        let direction = Vec3 {
            x: r.dir.x * self.cos_theta - r.dir.z * self.sin_theta,
            y: r.dir.y,
            z: r.dir.x * self.sin_theta + r.dir.z * self.cos_theta,
        };
        let rotated_r = Ray::new_tm(origin, direction, r.tm);

        match self.ptr.hit(&rotated_r, t_min, t_max) {
            None => None,
            Some(mut rec) => {
                let mut p = rec.p;
                let mut normal = rec.normal;
                p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
                p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

                normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
                normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

                rec.p = p;
                rec.set_face_normal(&rotated_r, normal);

                Some(rec)
            }
        }
    }
}

pub struct FlipFace {
    pub ptr: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(p: Arc<dyn Hittable>) -> FlipFace {
        FlipFace { ptr: p }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.ptr.hit(r, t_min, t_max) {
            None => None,
            Some(mut rec) => {
                rec.front_face = !rec.front_face;
                Some(rec)
            }
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.ptr.bounding_box(time0, time1)
    }
}
