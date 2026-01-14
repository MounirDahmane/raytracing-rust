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

/// Represents a volumetric medium with constant density for volumetric scattering,
/// typically used to simulate fog, smoke, or other participating media.
pub struct ConstantMedium {
    /// Boundary hittable defining the volume shape.
    boundary: Arc<dyn Hittable + Send + Sync>,
    /// Negative inverse of the density for scattering distance calculation.
    neg_inv_density: f64,
    /// Phase function material representing scattering properties inside the medium.
    phase_function: Arc<dyn Material + Send + Sync>,
}

impl ConstantMedium {
    /// Creates a new constant medium with given boundary, density, and texture.
    pub fn new(
        boundary: Arc<dyn Hittable + Send + Sync>,
        density: f64,
        tex: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_(tex)),
        }
    }

    /// Creates a new constant medium with given boundary, density, and albedo color.
    pub fn new_(boundary: Arc<dyn Hittable + Send + Sync>, density: f64, albedo: &Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    /// Checks if a ray hits the medium, simulating scattering inside the volume.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        // Find the intersections with the boundary.
        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        // Clamp intersections to ray interval.
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

        // Calculate distance the ray travels inside the medium.
        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (rtweekend::random_double()).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        // Record hit information.
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // Arbitrary normal
        rec.front_face = true; // Arbitrary front face
        rec.mat = Some(self.phase_function.clone());

        true
    }

    /// Returns the bounding box of the medium, same as the boundary's bounding box.
    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
