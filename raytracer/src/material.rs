#![allow(dead_code)]
#![allow(unused_variables)]
use crate::random_double_default;
use crate::*;

pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, pdf: &mut f64) -> Option<(Vec3, Ray)> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new()
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
    pub fn new_texture(a: Arc<dyn Texture + Send + Sync>) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new_tm(rec.p, unit_vector(scatter_direction), r_in.tm);
        let alb = self.albedo.value(rec.u, rec.v, rec.p);
        *pdf = dot(rec.normal, scattered.dir) / PI;

        Some((alb, scattered))
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(rec.normal, unit_vector(scattered.dir));
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Vec3, Ray)> {
        let reflected = reflect(unit_vector(r_in.dir), rec.normal);
        let scattered = Ray::new_tm(
            rec.p,
            reflected + random_in_unit_sphere() * self.fuzz,
            r_in.tm,
        );
        let attenuation = self.albedo;
        if dot(scattered.dir, rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
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
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Vec3, Ray)> {
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

        if refraction_ratio * sin_theta > 1.0
            || self.reflectance(cos_theta, refraction_ratio) > random_double_default()
        {
            Some((
                attenuation,
                Ray::new_tm(rec.p, reflect(unit_direction, rec.normal), r_in.tm),
            ))
        } else {
            Some((
                attenuation,
                Ray::new_tm(
                    rec.p,
                    refract(unit_direction, rec.normal, refraction_ratio),
                    r_in.tm,
                ),
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

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(c: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
    pub fn new_texture(a: Arc<dyn Texture + Send + Sync>) -> DiffuseLight {
        DiffuseLight { emit: a }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, pdf: &mut f64) -> Option<(Vec3, Ray)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    pub fn new_color(c: Color) -> Isotropic {
        Isotropic {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
    pub fn new(a: Arc<dyn Texture + Send + Sync>) -> Isotropic {
        Isotropic { albedo: a }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> Option<(Vec3, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, rec.p),
            Ray::new_tm(rec.p, random_in_unit_sphere(), r_in.tm),
        ))
    }
}
