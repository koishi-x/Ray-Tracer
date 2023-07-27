use crate::*;

#[derive(Clone)]
pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: M,
}

impl<M: Material> Sphere<M> {
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

        (phi / (2.0 * PI), theta / PI)
    }
}

impl<M: Material> Hittable for Sphere<M> {
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

        let mut rec = HitRecord::new(t, p, &self.mat_ptr);
        rec.set_face_normal(r, outward_normal);
        (rec.u, rec.v) = Self::get_sphere_uv(outward_normal);

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
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        match self.hit(&Ray::new(o, v), 0.001, INFINITY) {
            None => 0.0,
            Some(_) => {
                let cos_theta_max =
                    (1.0 - self.radius * self.radius / (self.center - o).length_squared()).sqrt();
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
        }
    }
    fn random(&self, o: Vec3) -> Vec3 {
        //println!("!!!!!\n");
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let uvw = ONB::build_from_w(direction);
        uvw.local_vec(random_to_sphere(self.radius, distance_squared))
    }
}
