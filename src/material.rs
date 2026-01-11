use crate::{texture::*, vec3::Point3};
use std::sync::Arc;

use crate::{
    color::{self, Color},
    hittable::HitRecord,
    ray::Ray,
    rtweekend,
    texture::Texture,
    vec3::Vec3,
};

pub struct NoMaterial;

/// Defines material behavior for scattering and emission.
/// Default implementation has no scattering or emission.
pub trait Material: Send + Sync {
    /// Calculates scattered ray and attenuation color.
    /// Returns true if scattering occurs, false otherwise.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    /// Returns emitted light color at a point.
    /// Default is black (no emission).
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

impl Material for NoMaterial {}

pub struct Lambertian {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    /// Creates Lambertian diffuse material with solid color.
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            tex: Arc::new(SolidColor::new(&albedo)),
        }
    }

    /// Creates Lambertian diffuse material with arbitrary texture.
    pub fn new_(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Lambertian { tex }
    }
}

impl Material for Lambertian {
    /// Scatters ray randomly around surface normal for diffuse reflection.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Avoid degenerate scatter direction near zero vector.
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);

        true
    }
}

pub struct Metal {
    albedo: color::Color,
    fuzz: f64,
}

impl Metal {
    /// Creates metal material with color and fuzziness (clamped to [0,1]).
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.min(1.0);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    /// Reflects ray with fuzz factor applied to scattered direction.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        let scattered_dir = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());

        *scattered = Ray::new(rec.p, scattered_dir, r_in.time());
        *attenuation = self.albedo;

        scattered.direction().dot(&rec.normal) > 0.0
    }
}

pub struct Dielectric {
    /// Refractive index of material.
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    /// Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = {
            let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
            r0 * r0
        };

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    /// Scatters ray by reflection or refraction based on Fresnel effect.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let reflect_prob = Dielectric::reflectance(cos_theta, ri);

        let direction = if cannot_refract || reflect_prob > rtweekend::random_double() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction, r_in.time());

        true
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex }
    }

    pub fn new_(emit: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    /// Returns emitted light color; does not scatter rays.
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    pub fn new(albedo: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    /// Scatters ray in random direction (used in volumetric media).
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
}
