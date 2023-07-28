use crate::*;

pub struct Triangle<M: Material> {
    pub a: Point3,
    pub n: Vec3,
    pub pb: Vec3,
    pub pc: Vec3,
    // pb / pc: perpendicular to ab / ac
    pub mat: M,
    pub bbox: AABB,
    pub uva: Vec3,
    pub uvab: Vec3,
    pub uvac: Vec3,
}
impl<M: Material> Triangle<M> {
    pub fn new(
        a: Point3,
        b: Point3,
        c: Point3,
        mat: M,
        (ua, va): (f64, f64),
        (ub, vb): (f64, f64),
        (uc, vc): (f64, f64),
    ) -> Self {
        let ab = b - a;
        let ac = c - a;
        let normal_ = ab ^ ac;
        let n = unit_vector(normal_);
        let det = normal_.length();
        let mut min = Point3::default();
        let mut max = Point3::default();
        // for i in 0..3 {
        //     min[i] = f64::min(f64::min(a[i], b[i]), c[i]) - 0.0001;
        //     max[i] = f64::max(f64::max(a[i], b[i]), c[i]) + 0.0001;
        // }
        min.x = f64::min(f64::min(a.x, b.x), c.x) - 0.0001;
        max.x = f64::max(f64::max(a.x, b.x), c.x) + 0.0001;

        min.y = f64::min(f64::min(a.y, b.y), c.y) - 0.0001;
        max.y = f64::max(f64::max(a.y, b.y), c.y) + 0.0001;

        min.z = f64::min(f64::min(a.z, b.z), c.z) - 0.0001;
        max.z = f64::max(f64::max(a.z, b.z), c.z) + 0.0001;
        let uva = Vec3 {
            x: ua,
            y: va,
            z: 0.,
        };
        let uvab = Vec3 {
            x: ub,
            y: vb,
            z: 0.,
        } - uva;
        let uvac = Vec3 {
            x: uc,
            y: vc,
            z: 0.,
        } - uva;

        Self {
            a,
            n,
            pb: (n ^ ab) / det,
            pc: (ac ^ n) / det,
            mat,
            bbox: AABB::new(min, max),
            uva,
            uvab,
            uvac,
        }
    }
}
impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = dot(self.a - r.orig, self.n) / dot(r.dir, self.n);
        if t < t_min || t_max < t {
            return None;
        }
        let p = r.at(t);
        let ap = p - self.a;
        let u = dot(ap, self.pc);
        let v = dot(ap, self.pb);
        // P = A + uAB + vAC
        if u >= 0. && v >= 0. && u + v <= 1. {
            let uv = self.uva + self.uvab * u + self.uvac * v;
            let mut rec = HitRecord::new(t, p, &self.mat);
            (rec.u, rec.v) = (uv.x, uv.y);
            rec.set_face_normal(r, self.n);
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}

// pub struct Triangle<M: Material> {
//     pub a: Point3,
//     pub b: Point3,
//     pub c: Point3,
//     pub normal: Vec3,
//     pub mat_ptr: M,
//     pub tc_a: (f64, f64),
//     pub tc_b: (f64, f64),
//     pub tc_c: (f64, f64),
//     pub na: Vec3,
//     pub nb: Vec3,
//     pub nc: Vec3,
// }

// impl<M: Material> Triangle<M> {
//     pub fn new(
//         a: Point3,
//         b: Point3,
//         c: Point3,
//         mat_ptr: M,
//         tc_a: (f64, f64),
//         tc_b: (f64, f64),
//         tc_c: (f64, f64),
//     ) -> Self {
//         Triangle {
//             a,
//             b,
//             c,
//             normal: unit_vector((b - a) ^ (c - a)),
//             mat_ptr,
//             tc_a,
//             tc_b,
//             tc_c,
//             na: Vec3::new(),
//             nb: Vec3::new(),
//             nc: Vec3::new(),
//         }
//     }
//     pub fn set_normal(&mut self, na: Vec3, nb: Vec3, nc: Vec3) {
//         self.na = na;
//         self.nb = nb;
//         self.nc = nc;
//     }

//     pub fn get_bc_coord(&self, p: Point3) -> Option<Vec3> {
//         let c = p - self.c;
//         let a = self.a - self.c;
//         let b = self.b - self.c;
//         let w1 = (c.x * b.y - c.y * b.x) / (a.x * b.y - a.y * b.x);
//         let w2 = (c.x * a.y - c.y * a.x) / (b.x * a.y - b.y * a.x);
//         if w1 < 0.0 || w2 < 0.0 || w1 + w2 > 1.0 {
//             None
//         } else {
//             Some(Vec3 {
//                 x: w1,
//                 y: w2,
//                 z: 1.0 - w1 - w2,
//             })
//         }
//     }
//     pub fn get_triangle_uv(&self, p: Point3) -> (f64, f64) {
//         match self.get_bc_coord(p) {
//             Some(coord) => (
//                 self.tc_a.0 * coord.x + self.tc_b.0 * coord.y + self.tc_c.0 * coord.z,
//                 self.tc_a.1 * coord.x + self.tc_b.1 * coord.y + self.tc_c.1 * coord.z,
//             ),
//             None => (0.0, 0.0),
//         }
//     }
// }

// impl<M: Material> Hittable for Triangle<M> {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         // let n_dot_dir = dot(self.normal, r.dir);
//         // if n_dot_dir == 0.0 {
//         //     return None;
//         // }
//         // let d = -dot(self.normal, self.a);
//         // let t = -(dot(self.normal, r.orig) + d) / n_dot_dir;
//         let t = dot(self.a - r.orig, self.normal) / dot(r.dir, self.normal);
//         if t < t_min || t > t_max {
//             return None;
//         }
//         let mut rec = HitRecord::new(t, r.at(t), &self.mat_ptr);
//         let coord = match self.get_bc_coord(rec.p) {
//             None => return None,
//             Some(c) => c,
//         };

//         // let outward_normal = if self.na.near_zero() && self.nb.near_zero() && self.nc.near_zero() {
//         //     self.normal
//         // } else {
//         //     unit_vector(self.na * coord.x + self.nb * coord.y + self.nc * coord.z)
//         // };
//         let outward_normal = self.normal;
//         rec.set_face_normal(r, outward_normal);
//         (rec.u, rec.v) = self.get_triangle_uv(rec.p);
//         Some(rec)
//     }
//     fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
//         let x_min = self.a.x.min(self.b.x.min(self.c.x));
//         let mut x_max = self.a.x.max(self.b.x.max(self.c.x));
//         let y_min = self.a.y.min(self.b.y.min(self.c.y));
//         let mut y_max = self.a.y.max(self.b.y.max(self.c.y));
//         let z_min = self.a.z.min(self.b.z.min(self.c.z));
//         let mut z_max = self.a.z.max(self.b.z.max(self.c.z));
//         Some(AABB {
//             minimum: Point3 {
//                 x: x_min - 0.0001,
//                 y: y_min - 0.0001,
//                 z: z_min - 0.0001,
//             },
//             maximum: Point3 {
//                 x: x_max + 0.0001,
//                 y: y_max + 0.0001,
//                 z: z_max + 0.0001,
//             },
//         })
//     }
//     fn pdf_value(&self, origin: Point3, v: Vec3) -> f64 {
//         match self.hit(&Ray::new(origin, v), 0.001, INFINITY) {
//             None => 0.0,
//             Some(rec) => {
//                 let area = ((self.b - self.a) ^ (self.c - self.a)).length() / 2.0;
//                 let distance_squared = rec.t * rec.t * v.length_squared();
//                 let cosine = (dot(v, rec.normal) / v.length()).abs();
//                 distance_squared / (cosine * area)
//             }
//         }
//     }
//     fn random(&self, o: Vec3) -> Vec3 {
//         println!("!!!");
//         loop {
//             let w1 = random_double_default();
//             let w2 = random_double_default();
//             if w1 + w2 > 1.0 {
//                 continue;
//             }
//             return self.a * w1 + self.b * w2 + self.c * (1.0 - w1 - w2) - o;
//         }
//     }
// }
