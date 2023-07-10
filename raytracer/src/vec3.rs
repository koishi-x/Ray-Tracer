use std::ops::{Add, AddAssign, BitXor, Div, Mul, MulAssign, Neg, Sub};

use crate::random_double;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    //type Output = ();
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl BitXor for Vec3 {
    //cross product
    type Output = Vec3;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

// impl Mul for Vec3 {
//     //dot product
//     type Output = f64;
//     fn mul(self, rhs: Self) -> Self::Output {
//         self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
//     }
// }

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        //self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn random(min: f64, max: f64) -> Vec3 {
    Vec3 {
        x: random_double(min, max),
        y: random_double(min, max),
        z: random_double(min, max),
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random(-1.0, 1.0);
        if p.length() >= 1.0 {
            continue;
        }
        return p;
    }
}

// pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
//     let in_unit_sphere = random_in_unit_sphere();
//     if dot(in_unit_sphere, normal) > 0.0 {
//         in_unit_sphere
//     } else {
//         -in_unit_sphere
//     }
// }

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3 {
            x: random_double(-1.0, 1.0),
            y: random_double(-1.0, 1.0),
            z: 0.0,
        };
        if p.length() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    //unit_vector(random_in_unit_sphere())
    let a = random_double(0.0, 2.0 * crate::PI);
    let z = random_double(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3 {
        x: r * a.cos(),
        y: r * a.sin(),
        z,
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * dot(n, v) * 2.0
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = -(n * (1.0 - r_out_perp.length_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}
