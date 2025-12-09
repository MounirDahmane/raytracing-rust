use rand::random;

use crate::{
    color::{self, Color},
    hittable::HitRecord,
    ray::{self, Ray},
    rtweekend,
    vec3::Vec3,
};

pub struct noMaterial;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}
impl Material for noMaterial {}

pub struct lambertian {
    albedo: color::Color,
}
impl lambertian {
    pub fn new(albedo: Color) -> Self {
        lambertian { albedo }
    }
}
impl Material for lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: color::Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.min(1.0);
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        reflected = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        return scattered.direction().dot(&rec.normal) > 0.0;
    }
}

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;

        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri: f64 = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        //  Schlick Approximation
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, ri) > rtweekend::random_double()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }
}
