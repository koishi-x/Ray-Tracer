use crate::*;

//pub use std::sync::Arc;
pub use std::rc::Rc;
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
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
    pub fn new(t: f64, p: Vec3, mat_ptr: &Rc<dyn Material>) -> HitRecord {
        HitRecord {
            p,
            normal: Vec3::new(),
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
