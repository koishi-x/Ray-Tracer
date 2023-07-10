use crate::*;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    // pub fn new() -> Self {
    //     Sphere {
    //         center: Vec3::new(),
    //         radius: 0.0,
    //     }
    // }
    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        // *u = phi / (2.0 * PI);
        // *v = theta / PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
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
        let outward_normal = (p - self.center) / self.radius;
        //let mat_ptr = self.mat_ptr;
        let mut rec = HitRecord::new(t, p, &self.mat_ptr);
        rec.set_face_normal(r, outward_normal);
        (rec.u, rec.v) = Sphere::get_sphere_uv(outward_normal);

        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            self.center
                - Vec3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
            self.center
                + Vec3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
        ))
    }
}
