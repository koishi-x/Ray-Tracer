use crate::*;

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub phase_function: Rc<dyn Material>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Rc::new(Isotropic::new(a)),
            neg_inv_density: -1.0 / d,
        }
    }
    pub fn new_color(b: Rc<dyn Hittable>, d: f64, c: Color) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Rc::new(Isotropic::new_color(c)),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1 = match self.boundary.hit(r, -INFINITY, INFINITY) {
            None => return None,
            Some(rec) => rec,
        };
        let mut rec2 = match self.boundary.hit(r, rec1.t + 0.0001, INFINITY) {
            None => return None,
            Some(rec) => rec,
        };
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double_default().log(std::f64::consts::E);
        if hit_distance > distance_inside_boundary {
            return None;
        }
        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        //Some(HitRecord { p, normal: Vec3 { x: 1.0, y: 1.0, z: 1.0 } , mat_ptr: self.phase_function, t, u: 0.0, v: 0.0, front_face: true })
        Some(HitRecord::new(t, p, &self.phase_function))
    }
}