use crate::{
    pdf::{CosinePdf, Pdf, SpherePdf},
    texture::*,
    vec3::Point3,
};
use std::sync::Arc;

use crate::{
    color::{self, Color},
    hittable::HitRecord,
    ray::Ray,
    rtweekend,
    texture::Texture,
    vec3::Vec3,
};

/// Stores the result of scattering a ray from a material.
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn Pdf + Send + Sync>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl ScatterRecord {
    /// Creates a new ScatterRecord with default values.
    pub fn new() -> Self {
        Self {
            attenuation: Color::default(),
            pdf_ptr: None,
            skip_pdf: false,
            skip_pdf_ray: Ray::default(),
        }
    }
}

/// A material that does nothing (no scattering or emission).
pub struct NoMaterial;

pub trait Material: Send + Sync {
    /// Compute scattering for an incoming ray.
    /// Returns true if scattering occurred and updates `srec`.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }

    /// Returns emitted light from the material at a given point.
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        Color::default()
    }

    /// Returns the PDF (probability density function) value for scattering in a given direction.
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
}

impl Material for NoMaterial {}

/// Lambertian (diffuse) material with texture.
pub struct Lambertian {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    /// Create a Lambertian material from a solid color.
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            tex: Arc::new(SolidColor::new(&albedo)),
        }
    }

    /// Create a Lambertian material from a texture.
    pub fn new_(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Lambertian { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePdf::new(&rec.normal)));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = Vec3::dot(&rec.normal, &Vec3::unit_vector(scattered.direction()));
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / rtweekend::PI
        }
    }
}

/// Metal material with fuzziness.
pub struct Metal {
    albedo: color::Color,
    fuzz: f64,
}

impl Metal {
    /// Create a metal material with given albedo and fuzz factor.
    /// Fuzz is clamped to a maximum of 1.0.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.min(1.0);
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let mut reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        reflected = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());

        srec.attenuation = self.albedo;
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new(rec.p, reflected, r_in.time());

        true
    }
}

/// Dielectric material representing transparent media (like glass).
pub struct Dielectric {
    /// Refractive index of the material.
    pub refraction_index: f64,
}

impl Dielectric {
    /// Create a new dielectric material with given refraction index.
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    /// Compute reflectance using Schlick's approximation.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.pdf_ptr = None;
        srec.skip_pdf = true;

        let ri: f64 = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        // Use Schlick approximation to decide reflection or refraction.
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, ri) > rtweekend::random_double()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        srec.skip_pdf_ray = Ray::new(rec.p, direction, r_in.time());

        true
    }
}

pub struct ColoredDielectric {
    pub refraction_index: f64,
    pub albedo: Color, // tinted color for the glass
}

impl ColoredDielectric {
    pub fn new(refraction_index: f64, albedo: Color) -> Self {
        Self {
            refraction_index,
            albedo,
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for ColoredDielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo; // use the tint here
        srec.pdf_ptr = None;
        srec.skip_pdf = true;

        let ri: f64 = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract
            || ColoredDielectric::reflectance(cos_theta, ri) > rtweekend::random_double()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        srec.skip_pdf_ray = Ray::new(rec.p, direction, r_in.time());

        true
    }
}

/// Material that emits light.
pub struct DiffuseLight {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    /// Create a new DiffuseLight from a texture.
    pub fn new(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex }
    }

    /// Create a new DiffuseLight from a solid color.
    pub fn new_(emit: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            return Color::default();
        }
        self.tex.value(u, v, p)
    }
}

/// Material for isotropic scattering (e.g., fog).
pub struct Isotropic {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    /// Create a new isotropic material from a solid color.
    pub fn new(albedo: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    /// Create a new isotropic material from a texture.
    pub fn new_(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(SpherePdf::new()));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * rtweekend::PI)
    }
}
