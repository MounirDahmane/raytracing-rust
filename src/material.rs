use crate::{onb::Onb, pdf::{CosinePdf, Pdf, SpherePdf}, texture::*, vec3::Point3};
use rand::random;
use std::rc::Rc;

use crate::{
    color::{self, Color},
    hittable::HitRecord,
    ray::{self, Ray},
    rtweekend,
    texture::Texture,
    vec3::Vec3,
};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Option<Rc<dyn Pdf>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}
impl ScatterRecord{
    pub fn new() -> Self {
        Self { attenuation: Color::default(), pdf_ptr: None, 
            skip_pdf: false, skip_pdf_ray: Ray::default() }
    }
}
pub struct noMaterial;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord, 
    ) -> bool {
        false
    }
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color{
        return Color::default();
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

}
impl Material for noMaterial {}

pub struct lambertian {
    tex: Rc<dyn Texture>,
}
impl lambertian {
    pub fn new(albedo: Color) -> Self {
        lambertian {
            tex: Rc::new(SolidColor::new(&albedo)),
        }
    }
    pub fn new_(tex: Rc<dyn Texture>) -> Self {
        lambertian { tex }
    }
}
impl Material for lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> bool {

        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Rc::new(CosinePdf::new(&rec.normal)));
        srec.skip_pdf = false;
        return true;
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = Vec3::dot(&rec.normal, &Vec3::unit_vector(scattered.direction()));
        if cos_theta < 0.0 {return 0.0;}
        else {
            return cos_theta / rtweekend::PI;
        }
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
        srec: &mut ScatterRecord,
    ) -> bool {
        let mut reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        reflected = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());

        srec.attenuation = self.albedo;
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new(rec.p, reflected, r_in.time());

        return true;
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
        srec: &mut ScatterRecord,
    ) -> bool {
        
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

        //  Schlick Approximation
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

pub struct DiffuseLight{
    tex: Rc<dyn Texture>,
}
impl DiffuseLight {
   
    pub fn new(tex: Rc<dyn Texture>) -> Self {
        Self { tex }
    }
    pub fn new_(emit: &Color) -> Self {
        Self { tex: Rc::new(SolidColor::new(emit)) }
    }
}
impl Material for DiffuseLight {
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color{
        if !rec.front_face {
            return Color::default();
        }
        return self.tex.value(u, v, p);
    }
}

pub struct Isotropic {
    tex: Rc<dyn Texture>,
}
impl Isotropic {
    
    pub fn new(albedo: &Color) -> Self {
        Self { tex: Rc::new(SolidColor::new(albedo)) }
    }
    pub fn new_(tex: Rc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {

        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Rc::new(SpherePdf::new()));
        srec.skip_pdf = false;

        return true;
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        1.0 / (4.0 * rtweekend::PI)
    }
}