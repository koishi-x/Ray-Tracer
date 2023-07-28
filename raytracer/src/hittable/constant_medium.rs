use crate::*;

pub struct ConstantMedium<H: Hittable, M: Material> {
    pub boundary: H,
    pub phase_function: M,
    pub neg_inv_density: f64,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, Isotropic<T>> {
    pub fn new(b: H, d: f64, a: T) -> Self {
        ConstantMedium {
            boundary: b,
            phase_function: Isotropic::new(a),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<H: Hittable> ConstantMedium<H, Isotropic<SolidColor>> {
    pub fn new_color(b: H, d: f64, c: Color) -> Self {
        ConstantMedium {
            boundary: b,
            phase_function: Isotropic::new_color(c),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<H: Hittable, M: Material> Hittable for ConstantMedium<H, M> {
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
