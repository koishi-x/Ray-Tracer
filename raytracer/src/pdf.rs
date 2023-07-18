use crate::*;

//#[allow(clippy::upper_case_acronyms)]
pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double_default();
    let r2 = random_double_default();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3 { x, y, z }
}

pub struct CosinePdf {
    pub uvw: ONB,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = dot(unit_vector(direction), self.uvw.w());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(random_cosine_direction())
    }
}
