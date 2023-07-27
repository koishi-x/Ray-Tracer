use crate::*;

use image::{open, DynamicImage, GenericImageView};
use std::mem::swap;

const BYTES_PER_PIXEL: u32 = 3;
pub struct YZImageBox {
    box_min: Point3,
    box_max: Point3,
    bytes_per_scanline: u32,
    data: Vec<u8>,
    width: u32,
    height: u32,
    mat_ptr: Vec<Vec<DiffuseLight<SolidColor>>>,
}
impl YZImageBox {
    pub fn new(box_min: Point3, box_max: Point3, filename: &str, scale: f64) -> YZImageBox {
        let image_result: Result<DynamicImage, _> = open(filename);
        let mut data: Vec<u8> = Vec::new();
        let mut bytes_per_scanline = 0;
        let mut width = 0;
        let mut height = 0;
        let mut mat_ptr: Vec<Vec<DiffuseLight<SolidColor>>> = Vec::new();
        match image_result {
            Ok(image) => {
                (width, height) = image.dimensions();
                bytes_per_scanline = BYTES_PER_PIXEL * width;
                for y in 0..height {
                    mat_ptr.push(Vec::new());
                    for x in 0..width {
                        let pixel = image.get_pixel(x, y);
                        data.push(pixel[0]);
                        data.push(pixel[1]);
                        data.push(pixel[2]);
                        mat_ptr[y as usize].push(DiffuseLight::new(
                            Vec3 {
                                x: pixel[0] as f64 / 256.0,
                                y: pixel[1] as f64 / 256.0,
                                z: pixel[2] as f64 / 256.0,
                            } * scale,
                        ))
                    }
                }
            }
            Err(_err) => {}
        }
        YZImageBox {
            box_min,
            box_max,
            data,
            width,
            height,
            bytes_per_scanline,
            mat_ptr,
        }
    }
    fn check(&self, y: u32, z: u32) -> Option<HitRecord> {
        let pos = (y * self.bytes_per_scanline + z * BYTES_PER_PIXEL) as usize;
        let c = Color {
            x: self.data[pos] as f64 / 256.0,
            y: self.data[pos + 1] as f64 / 256.0,
            z: self.data[pos + 2] as f64 / 256.0,
        };
        if c.length_squared() < 0.5 {
            None
        } else {
            //let tmp = DiffuseLight::new(c);
            let mut ret = HitRecord::new(0.0, Point3::new(), &self.mat_ptr[y as usize][z as usize]);
            ret.front_face = true;
            Some(ret)
        }
    }
}
impl Hittable for YZImageBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut inv_d = 1.0 / r.dir.x;
        let mut t0 = (self.box_min.x - r.orig.x) / r.dir.x;
        let mut t1 = (self.box_max.x - r.orig.x) / r.dir.x;
        let mut min = t_min;
        let mut max = t_max;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        min = min.max(t0);
        max = max.min(t1);
        if max <= min {
            return None;
        }
        inv_d = 1.0 / r.dir.y;
        t0 = (self.box_min.y - r.orig.y) / r.dir.y;
        t1 = (self.box_max.y - r.orig.y) / r.dir.y;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        min = min.max(t0);
        max = max.min(t1);
        if max <= min {
            return None;
        }
        inv_d = 1.0 / r.dir.z;
        t0 = (self.box_min.z - r.orig.z) / r.dir.z;
        t1 = (self.box_max.z - r.orig.z) / r.dir.z;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        min = min.max(t0);
        max = max.min(t1);
        if max <= min {
            return None;
        }

        let p0 = r.at(min);
        let p1 = r.at(max);
        let mut y0 = (p0.y - self.box_max.y) / (self.box_min.y - self.box_max.y);
        let mut z0 = (p0.z - self.box_max.z) / (self.box_min.z - self.box_max.z);
        let ymin = (clamp(y0, 0.0, 0.999) * self.height as f64) as u32;
        let ymax = (clamp(
            (p1.y - self.box_max.y) / (self.box_min.y - self.box_max.y),
            0.0,
            0.999,
        ) * self.height as f64) as u32;
        let mut z = (clamp(z0, 0.0, 0.999) * self.width as f64) as u32;
        let zmax = (clamp(
            (p1.z - self.box_max.z) / (self.box_min.z - self.box_max.z),
            0.0,
            0.999,
        ) * self.width as f64) as u32;
        y0 *= self.height as f64;
        z0 *= self.width as f64;
        let mut y1 = (p1.y - self.box_max.y) / (self.box_min.y - self.box_max.y);
        let mut z1 = (p1.z - self.box_max.z) / (self.box_min.z - self.box_max.z);
        y1 *= self.height as f64;
        z1 *= self.width as f64;
        y1 -= y0;
        z1 -= z0;
        if p0.y > p1.y {
            for y in ymin..(ymax + 1) {
                match self.check(y, z) {
                    Some(ret) => {
                        return Some(ret);
                    }
                    None => {}
                }
                if p0.z > p1.z {
                    while z < zmax
                        && (((y + 1) as f64 - y0) * z1 - (z as f64 - z0) * y1)
                            * (((y + 1) as f64 - y0) * z1 - ((z + 1) as f64 - z0) * y1)
                            > std::f64::EPSILON
                    {
                        z += 1;
                        match self.check(y, z) {
                            Some(ret) => {
                                return Some(ret);
                            }
                            None => {}
                        }
                    }
                } else {
                    while z > zmax
                        && (((y + 1) as f64 - y0) * z1 - (z as f64 - z0) * y1)
                            * (((y + 1) as f64 - y0) * z1 - ((z + 1) as f64 - z0) * y1)
                            > std::f64::EPSILON
                    {
                        z -= 1;
                        match self.check(y, z) {
                            Some(ret) => {
                                return Some(ret);
                            }
                            None => {}
                        }
                    }
                }
            }
        } else {
            for y in (ymax..(ymin + 1)).rev() {
                match self.check(y, z) {
                    Some(ret) => {
                        return Some(ret);
                    }
                    None => {}
                }
                if p0.z > p1.z {
                    while z < zmax
                        && ((y as f64 - y0) * z1 - (z as f64 - z0) * y1)
                            * ((y as f64 - y0) * z1 - ((z + 1) as f64 - z0) * y1)
                            > std::f64::EPSILON
                    {
                        z += 1;
                        match self.check(y, z) {
                            Some(ret) => {
                                return Some(ret);
                            }
                            None => {}
                        }
                    }
                } else {
                    while z > zmax
                        && ((y as f64 - y0) * z1 - (z as f64 - z0) * y1)
                            * ((y as f64 - y0) * z1 - ((z + 1) as f64 - z0) * y1)
                            > std::f64::EPSILON
                    {
                        z -= 1;
                        match self.check(y, z) {
                            Some(ret) => {
                                return Some(ret);
                            }
                            None => {}
                        }
                    }
                }
            }
        }
        if p0.z < p1.z {
            while z < zmax {
                z += 1;
                match self.check(ymax, z) {
                    Some(ret) => {
                        return Some(ret);
                    }
                    None => {}
                }
            }
        } else {
            while z > zmax {
                z -= 1;
                match self.check(ymax, z) {
                    Some(ret) => {
                        return Some(ret);
                    }
                    None => {}
                }
            }
        }
        None
    }
    fn bounding_box(&self, _timx: f64, _time1: f64) -> Option<AABB> {
        let ret = AABB::new(self.box_min, self.box_max);
        Some(ret)
    }
}

// use image::GenericImageView;

// use crate::*;

// pub struct ImageBox {
//     pub box_min: Point3,
//     pub box_max: Point3,
//     pub sides: HittableList,
//     // data: image::DynamicImage,
//     // width: u32,
//     // height: u32,
//     pub tex: ImageTexture,
// }

// impl ImageBox {
//     pub fn new<M: Material + Clone + 'static>(
//         p0: Point3,
//         p1: Point3,
//         ptr: M,
//         filename: &str,
//     ) -> Self {
//         let mut sides = HittableList::new();
//         sides.add(Arc::new(XYRect::new(
//             p0.x,
//             p1.x,
//             p0.y,
//             p1.y,
//             p1.z,
//             ptr.clone(),
//         )));
//         sides.add(Arc::new(XYRect::new(
//             p0.x,
//             p1.x,
//             p0.y,
//             p1.y,
//             p0.z,
//             ptr.clone(),
//         )));

//         sides.add(Arc::new(XZRect::new(
//             p0.x,
//             p1.x,
//             p0.z,
//             p1.z,
//             p1.y,
//             ptr.clone(),
//         )));
//         sides.add(Arc::new(XZRect::new(
//             p0.x,
//             p1.x,
//             p0.z,
//             p1.z,
//             p0.y,
//             ptr.clone(),
//         )));

//         sides.add(Arc::new(YZRect::new(
//             p0.y,
//             p1.y,
//             p0.z,
//             p1.z,
//             p1.x,
//             ptr.clone(),
//         )));
//         sides.add(Arc::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr)));

//         let data = image::open(filename).expect("Cannot open.");
//         let (width, height) = image::GenericImageView::dimensions(&data);

//         Self {
//             box_min: p0,
//             box_max: p1,
//             sides,
//             tex: ImageTexture::new(filename),
//         }
//     }
//     pub fn get_uv(&self, p: Point3) -> (u32, u32) {
//         (
//             (((p.x - self.box_min.x) / (self.box_max.x - self.box_min.x)) as u32 * self.width)
//                 .min(self.tex.width - 1)
//                 + 1,
//             (((p.y - self.box_min.y) / (self.box_max.y - self.box_min.y)) as u32 * self.height)
//                 .min(self.tex.height - 1)
//                 + 1,
//         )
//     }
//     pub fn locate(&self, u: u32, v: u32) -> Point3 {
//         Point3 {
//             z: 0.0,
//             x: self.box_min.x + (self.box_max.x - self.box_min.x) * u as f64 / self.width as f64,
//             y: self.box_min.y + (self.box_max.y - self.box_min.y) * v as f64 / self.height as f64,
//         }
//     }
//     pub fn get_pixel_color(&self, u: u32, v: u32) -> Color {
//         let pixel = self.tex.data.get_pixel(u, v);
//         let color_scale = 1.0 / 255.0;
//         Color {
//             x: pixel[0] as f64 * color_scale,
//             y: pixel[1] as f64 * color_scale,
//             z: pixel[2] as f64 * color_scale,
//         }
//     }
// }

// fn diff_z_flag(a: Vec3, b: Vec3) -> bool {
//     if (a.z >= 0.0 && b.z <= 0.0) || (a.z <= 0.0 && b.z >= 0.0) {
//         true
//     } else {
//         false
//     }
// }

// impl Hittable for ImageBox {
//     fn bounding_box(&self, _timx: f64, _time1: f64) -> Option<AABB> {
//         Some(AABB::new(self.box_min, self.box_max))
//     }
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         match self.sides.hit(r, t_min, t_max) {
//             Some(rec1) => match self.sides.hit(r, rec1.t + 0.001, t_max) {
//                 Some(rec2) => {
//                     let mut res_t = 0.0;
//                     let mut res_color = Color::new();
//                     let is_hit = false;
//                     for i in 0..self.tex.width {
//                         for j in 0..self.tex.height {
//                             let pixel = self.get_pixel_color(i, j);
//                             if pixel.length_squared() < 0.5 {
//                                 continue;
//                             }

//                             let mut p0 = self.locate(i, j);
//                             p0.z = self.box_min.z;
//                             let mut p1 = self.locate(i + 1, j + 1);
//                             p1.z = self.box_max.z;
//                             let tmp = AABox::new(p0, p1, DefaultMaterial {});
//                             match tmp.hit(r, t_min, t_max) {
//                                 Some(rec) => {
//                                     if !is_hit || rec.t < res_t {
//                                         res_t = rec.t;
//                                         res_color = pixel;
//                                     }
//                                 }
//                                 None => {}
//                             }
//                         }
//                     }
//                     if is_hit {
//                         let qwq = Some(HitRecord::new(
//                             res_t,
//                             r.at(res_t),
//                             &Lambertian::new_texture(self.tex),
//                         ));
//                         (qwq.u, qwq.v) =
//                     } else {
//                         None
//                     }
//                     // let p1 = r.at(rec1.t);
//                     // let p2 = r.at(rec2.t);
//                     // let k = r.dir.y / r.dir.x;
//                     // let (u, v) = self.get_uv(p1);
//                     // if k >= 0.0 {
//                     //     if r.dir.x >= 0.0 {
//                     //         for j in (1..=v).rev() {
//                     //             for i in 0..self.width {
//                     //                 let mut v1 = self.locate(i, j - 1) - p1;
//                     //                 let mut v2 = self.locate(i + 1, j) - p1;
//                     //                 v1.z = 0.0;
//                     //                 v2.z = 0.0;
//                     //                 let mut tmp = r.dir;
//                     //                 tmp.z = 0.0;
//                     //                 if diff_z_flag(tmp ^ v1, tmp ^ v2) {
//                     //                     //let pixel = self.data.get_pixel(i, j - 1);
//                     //                     let pixel = self.get_pixel_color(i, j - 1);
//                     //                     if pixel.length_squared() >= 0.5 {
//                     //                         let rec =
//                     //                     }
//                     //                 }
//                     //             }
//                     //         }
//                     //     }
//                     // }
//                     // None
//                 }
//                 None => None,
//             },
//             None => None,
//         }
//     }
// }
