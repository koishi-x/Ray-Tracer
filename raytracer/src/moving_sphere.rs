use crate::*;

pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center(r.tm);
        let a = r.dir.length_squared();
        let half_b = dot(oc, r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center(r.tm)) / self.radius;
        //let mat_ptr = self.mat_ptr;
        let mut rec = HitRecord::new(t, p, &self.mat_ptr);
        //     {
        //     t,
        //     p,
        //     mat_ptr: self.mat_ptr.clone(),
        //     normal: outward_normal,
        //     front_face: false,
        // };
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(time0)
                - Vec3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
            self.center(time0)
                + Vec3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
        );
        let box1 = AABB::new(
            self.center(time1)
                - Vec3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
            self.center(time1)
                + Vec3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
        );
        Some(surrounding_box(&box0, &box1))
    }
}
