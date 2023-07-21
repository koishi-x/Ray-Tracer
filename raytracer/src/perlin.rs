#![allow(dead_code)]

use crate::*;

#[derive(Clone)]
pub struct Perlin {
    //ranfloat: Vec<f64>,
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranvec = Vec::with_capacity(Self::POINT_COUNT);
        for _ in 0..Self::POINT_COUNT {
            ranvec.push(unit_vector(random(-1.0, 1.0)));
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        // let i = ((4.0 * p.x) as i32 & 255) as usize;
        // let j = ((4.0 * p.y) as i32 & 255) as usize;
        // let k = ((4.0 * p.z) as i32 & 255) as usize;

        // self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::new(); 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cij) in ci.iter_mut().enumerate() {
                for (dk, cijk) in cij.iter_mut().enumerate() {
                    *cijk = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, mut p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }
        accum.abs()
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = vec![0; Self::POINT_COUNT];

        for (i, x) in p.iter_mut().enumerate().take(Self::POINT_COUNT) {
            *x = i;
        }

        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut [usize]) {
        let n = p.len();

        for i in (1..n).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;

        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    let weight_v = Vec3 {
                        x: u - i as f64,
                        y: v - j as f64,
                        z: w - k as f64,
                    };
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * dot(*cijk, weight_v);
                }
            }
        }
        accum
    }
}
