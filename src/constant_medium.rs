use std::sync::Arc;

use crate::color::Color;
use crate::material::Isotropic;
use crate::{
    Ray,
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    rtweekend::{self, INFINITY},
    texture::Texture,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,       // Shape defining the volume boundary
    neg_inv_density: f64,              // Negative inverse of medium density, for sampling
    phase_function: Arc<dyn Material>, // Scattering phase function (isotropic)
}

impl ConstantMedium {
    /// Create a constant medium volume with given boundary, density, and texture.
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_(tex)),
        }
    }

    /// Alternative constructor with albedo color instead of texture.
    pub fn new_(boundary: Arc<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        // Find entry point of ray into boundary volume (infinite interval)
        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        // Find exit point of ray from boundary volume after entry
        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        // Clamp intersections to the allowed ray interval
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;

        // Sample a random scattering distance inside the medium based on density
        let hit_distance = self.neg_inv_density * (rtweekend::random_double()).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        // Record hit point along the ray inside the medium
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        // Set arbitrary normal and face orientation (volumetric medium has no surface normal)
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
