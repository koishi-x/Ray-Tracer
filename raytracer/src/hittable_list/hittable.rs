//use crate::{Ray, Vec3};
use crate::{material::*, ray::*, vec3::*};

pub use std::sync::Arc;
pub trait Hittable {
    //fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
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
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.dir, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
