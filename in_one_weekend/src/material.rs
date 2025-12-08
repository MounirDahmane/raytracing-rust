use crate::{color::{self, Color}, hittable::HitRecord, ray::{self, Ray}, vec3::Vec3};

pub struct noMaterial;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        false
    }
}
impl Material for noMaterial{}


pub struct lambertian{
    albedo: color::Color,
}

impl lambertian {
    pub fn new(albedo: Color) -> Self{
        lambertian { albedo }
    }
}
impl Material for lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero(){
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
        Metal { albedo, fuzz}
    }
}
impl Material for Metal {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool{
        let mut reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        reflected = Vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        return scattered.direction().dot(&rec.normal) > 0.0;
    }
}
