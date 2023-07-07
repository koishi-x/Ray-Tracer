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

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };

        let unit_direction = unit_vector(r_in.dir);
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if refraction_ratio * sin_theta > 1.0 {
            Some((
                attenuation,
                Ray {
                    orig: rec.p,
                    dir: reflect(unit_direction, rec.normal),
                },
            ))
        } else {
            Some((
                attenuation,
                Ray {
                    orig: rec.p,
                    dir: refract(unit_direction, rec.normal, refraction_ratio),
                },
            ))
        }

        // let refracted = refract(unit_direction, rec.normal, refraction_ratio);

        // let scattered = Ray {
        //     orig: rec.p,
        //     dir: refracted,
        // };
        // Some((attenuation, scattered))
    }
}
