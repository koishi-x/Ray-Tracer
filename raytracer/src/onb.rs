use crate::*;

#[allow(clippy::upper_case_acronyms)]
pub struct ONB {
    pub axis: [Vec3; 3],
}

impl ONB {
    pub fn new() -> ONB {
        ONB {
            axis: [Vec3::new(), Vec3::new(), Vec3::new()],
        }
    }
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u() * a + self.v() * b + self.w() * c
    }
    pub fn local_vec(&self, a: Vec3) -> Vec3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }

    pub fn build_from_w(n: Vec3) -> ONB {
        let w = unit_vector(n);
        let a = if w.x > 0.9 {
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        } else {
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        };
        let v = unit_vector(w ^ a);
        let u = w ^ v;
        ONB { axis: [u, v, w] }
    }
}
