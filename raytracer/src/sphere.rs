use crate::{hittable_list::*, vec3::*, Material, Ray};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    // pub fn new() -> Self {
    //     Sphere {
    //         center: Vec3::new(),
    //         radius: 0.0,
    //     }
    // }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = dot(r.dir, r.dir);
        let half_b = dot(oc, r.dir);
        let c = dot(oc, oc) - self.radius * self.radius;
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
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        //let mat_ptr = self.mat_ptr;
        let mut rec = HitRecord {
            t,
            p,
            mat_ptr: self.mat_ptr.clone(),
            normal: outward_normal,
            front_face: false,
        };
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
    // fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    //     let oc = r.orig - self.center;
    //     let a = r.dir * r.dir;
    //     let half_b = oc * r.dir;
    //     let c = oc * oc - self.radius * self.radius;
    //     let discriminant = half_b * half_b - a * c;
    //     if discriminant < 0.0 {
    //         return false;
    //     }

    //     let sqrtd = discriminant.sqrt();
    //     let mut root = (-half_b - sqrtd) / a;
    //     if root < t_min || t_max < root {
    //         root = (-half_b + sqrtd) / a;
    //         if root < t_min || t_max < root {
    //             return false;
    //         }
    //     }
    //     rec.t = root;
    //     rec.p = r.at(root);
    //     //rec.normal = (rec.p - self.center) / self.radius;
    //     let outward_normal = (rec.p - self.center) / self.radius;
    //     rec.set_face_normal(r, outward_normal);
    //     rec.mat_ptr = self.mat_ptr;
    //     true
    // }
}
