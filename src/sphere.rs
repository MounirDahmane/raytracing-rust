use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::rtweekend::PI;
use std::rc::Rc;

/// Represents a sphere, which can be static or moving, with a material and bounding box.
pub struct Sphere {
    center: Ray,           // Represents the center position and velocity (for moving spheres).
    radius: f64,           // Sphere radius, non-negative.
    mat: Rc<dyn Material>, // Shared reference to the sphere's material.
    bbox: AABB,            // Axis-aligned bounding box for the sphere.
}

impl Sphere {
    /// Creates a new static sphere with fixed center and radius.
    #[inline(always)]
    pub fn new_static_sphere(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::new_from_points(static_center - rvec, static_center + rvec);

        Sphere {
            center: Ray::new_no_time(static_center, Vec3::default()),
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }

    /// Creates a new moving sphere with linear interpolation between two centers over time.
    #[inline(always)]
    pub fn new_dynamic_sphere(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        let center = Ray::new_no_time(center1, center2 - center1);
        let radius = radius.max(0.0);

        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::new_from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = AABB::new_from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        let bbox = AABB::new_(box1, box2);

        Sphere {
            center,
            radius,
            mat,
            bbox,
        }
    }
}

impl Sphere {
    /// Computes the UV texture coordinates on a unit sphere at point `p`.
    ///
    /// - `p`: point on the unit sphere (centered at origin).
    /// - `u`: mutable reference to store horizontal coordinate in [0,1].
    /// - `v`: mutable reference to store vertical coordinate in [0,1].
    ///
    /// Explanation:
    /// - `u` is angle around the Y axis from X = -1 (left).
    /// - `v` is angle from Y = -1 (bottom) to Y = +1 (top).
    pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    /// Determines if a ray hits the sphere between `ray_t` interval.
    ///
    /// Updates `rec` with hit information if true.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root within the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);

        rec.mat = Some(Rc::clone(&self.mat));

        true
    }

    /// Returns the bounding box of the sphere.
    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
