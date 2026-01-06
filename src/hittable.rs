use crate::material::Material;
use crate::vec3::{Point3, Vec3};
use crate::{interval::Interval, Ray};
use std::rc::Rc;

pub struct HitRecord {
    /// Point of intersection.
    pub p: Point3,
    /// Surface normal at the intersection point.
    pub normal: Vec3,
    /// Reference to the material of the hit object.
    pub mat: Option<Rc<dyn Material>>,
    /// Ray parameter `t` at intersection.
    pub t: f64,
    /// True if the ray hits the front face of the surface.
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the normal vector and front_face flag depending on ray direction.
    ///
    /// `outward_normal` must be a unit vector.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: None,
            t: 0.0,
            front_face: false,
        }
    }
}

/// Trait for hittable objects in the scene.
pub trait Hittable {
    /// Returns true if ray hits the object between ray_t.min and ray_t.max,
    /// updating `rec` with hit details.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
