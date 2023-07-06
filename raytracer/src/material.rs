use raytracer::clamp;

//use hittable_list::*;
use crate::{vec3::*, HitRecord, Ray};
pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray {
            orig: rec.p,
            dir: scatter_direction,
        };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Vec3, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: clamp(f, 0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(unit_vector(r_in.dir), rec.normal);
        let scattered = Ray {
            orig: rec.p,
            dir: reflected + random_in_unit_sphere() * self.fuzz,
        };
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
