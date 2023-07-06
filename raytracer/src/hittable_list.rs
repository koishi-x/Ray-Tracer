//pub use std::rc::Rc;
pub use std::vec::Vec;

pub mod hittable;

use crate::ray::*;

pub use hittable::*;
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //let mut temp_rec = HitRecord::new();
        //let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(r, t_min, closest_so_far) {
                None => (),
                Some(temp_rec) => {
                    //hit_anything = true;
                    closest_so_far = temp_rec.t;
                    //rec = temp_rec;
                    rec = Some(temp_rec);
                } // hit_anything = true;
                  // closest_so_far = temp_rec.t;
                  // *rec = temp_rec;
            }
        }
        rec
    }
    // fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    //     let mut temp_rec = HitRecord::new();
    //     let mut hit_anything = false;
    //     let mut closest_so_far = t_max;

    //     for object in &self.objects {
    //         if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
    //             hit_anything = true;
    //             closest_so_far = temp_rec.t;
    //             *rec = temp_rec;
    //         }
    //     }

    //     hit_anything
    // }
}
