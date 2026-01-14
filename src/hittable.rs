use crate::aabb::Aabb;
use crate::material::Material;
use crate::rtweekend::{self, INFINITY};
use crate::vec3::{Point3, Vec3};
use crate::{Ray, interval::Interval};
use std::sync::Arc;

/// Stores information about a ray-object hit.
pub struct HitRecord {
    /// Point of intersection.
    pub p: Point3,
    /// Surface normal at the intersection.
    pub normal: Vec3,
    /// Material at the intersection point.
    pub mat: Option<Arc<dyn Material + Send + Sync>>,
    /// Ray parameter at hit point.
    pub t: f64,
    /// Texture coordinates u.
    pub u: f64,
    /// Texture coordinates v.
    pub v: f64,
    /// True if ray hits front face, false if inside surface.
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the normal vector and front face flag depending on ray direction.
    ///
    /// `outward_normal` must be unit length.
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
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

/// Trait for objects that can be intersected by rays.
pub trait Hittable: Send + Sync {
    /// Checks for ray intersection within a given interval.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    /// Returns the bounding box enclosing the object.
    fn bounding_box(&self) -> Aabb;

    /// Probability density function value for a given ray origin and direction.
    /// Defaults to 0.0 (no importance sampling).
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        0.0
    }

    /// Returns a random direction vector for importance sampling.
    /// Defaults to a fixed vector.
    fn random(&self, origin: &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

/// Translates a hittable object by a given offset.
pub struct Translate {
    object: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    /// Creates a new translated hittable object.
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    /// Checks for ray intersection, offsetting the ray by negative offset.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }
        rec.p += self.offset;
        true
    }

    /// Returns the translated bounding box.
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

/// Rotates a hittable object around the Y-axis by a specified angle.
pub struct RotateY {
    object: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    /// Creates a new Y-axis rotated hittable object with given angle in degrees.
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {
        let radians = rtweekend::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        // Compute rotated bounding box corners
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.x.max + ((1 - i) as f64) * bbox.x.min;
                    let y = (j as f64) * bbox.y.max + ((1 - j) as f64) * bbox.y.min;
                    let z = (k as f64) * bbox.z.max + ((1 - k) as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = Aabb::new_from_points(min, max);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    /// Checks for ray intersection after rotating ray into object space.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Rotate ray origin and direction into object space
        let origin = Point3::new(
            self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z(),
            r.origin().y(),
            self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z(),
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z(),
            r.direction().y(),
            self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z(),
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // Rotate intersection point and normal back to world space
        rec.p = Point3::new(
            self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z(),
            rec.p.y(),
            -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z(),
        );

        rec.normal = Vec3::new(
            self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
            rec.normal.y(),
            -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
        );

        true
    }

    /// Returns the bounding box of the rotated object.
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
