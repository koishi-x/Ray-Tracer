#![allow(dead_code)]

use crate::*;
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut ranfloat = Vec::with_capacity(Self::POINT_COUNT);
        for _ in 0..Self::POINT_COUNT {
            ranfloat.push(random_double_default());
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranfloat,
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

        let mut c = [[[0.0; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cij) in ci.iter_mut().enumerate() {
                for (dk, cijk) in cij.iter_mut().enumerate() {
                    *cijk = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        Self::trilinear_interp(&c, u, v, w)
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * cijk;
                }
            }
        }
        accum
    }
}
