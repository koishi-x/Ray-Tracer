use crate::*;

pub mod hittable;
pub use hittable::*;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(r, t_min, closest_so_far) {
                None => (),
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    rec = Some(temp_rec);
                }
            }
        }
        rec
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        //let mut first_box = true;
        let mut output_box: Option<AABB> = None;

        for object in &self.objects {
            match object.bounding_box(time0, time1) {
                None => return None,
                Some(temp_box) => {
                    output_box = match output_box {
                        None => Some(temp_box),
                        Some(last_box) => Some(surrounding_box(&temp_box, &last_box)),
                    }
                }
            }
        }
        output_box
    }
}
