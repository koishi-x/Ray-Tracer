use crate::*;

pub mod onb;
pub use onb::*;
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

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_double_default();
    let r2 = random_double_default();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vec3 { x, y, z }
}

pub struct DefaultPdf {}

impl Pdf for DefaultPdf {
    fn value(&self, _direction: Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::new()
    }
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

pub struct HittablePdf<'a, H: Hittable> {
    o: Point3,
    ptr: &'a H,
}

impl<'a, H: Hittable> HittablePdf<'a, H> {
    pub fn new(p: &'a H, origin: Point3) -> Self {
        HittablePdf { o: origin, ptr: p }
    }
}

impl<'a, H: Hittable> Pdf for HittablePdf<'a, H> {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}

pub struct MixturePdf<'a> {
    //pub p: [Arc<dyn Pdf>; 2],
    pub p0: &'a dyn Pdf,
    pub p1: &'a dyn Pdf,
}

impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> Self {
        MixturePdf { p0, p1 }
    }
}

impl<'a> Pdf for MixturePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double_default() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
