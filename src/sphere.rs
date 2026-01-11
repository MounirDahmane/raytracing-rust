use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::*;
use crate::ray::Ray;
use crate::rtweekend::PI;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

/// Represents a sphere, which can be static or moving, with a material and bounding box.
pub struct Sphere {
    center: Ray, // Center position and velocity for moving spheres.
    radius: f64, // Sphere radius, non-negative.
    mat: Arc<dyn Material + Send + Sync>, // Material reference.
    bbox: AABB,  // Bounding box enclosing the sphere.
}

impl Sphere {
    /// Creates a new static sphere with fixed center and radius.
    #[inline(always)]
    pub fn new_static_sphere(
        static_center: Point3,
        radius: f64,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> Self {
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
        mat: Arc<dyn Material + Send + Sync>,
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
    /// Computes UV texture coordinates for a point on the unit sphere.
    ///
    /// `p`: Point on the unit sphere (centered at origin).
    /// `u`: Horizontal coordinate [0,1] around Y axis.
    /// `v`: Vertical coordinate [0,1] from bottom (-Y) to top (+Y).
    pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    /// Checks if ray `r` hits the sphere within `ray_t` interval.
    ///
    /// If hit, updates `rec` with intersection details.
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

        // Find nearest root in acceptable range.
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

        rec.mat = Some(Arc::clone(&self.mat));

        true
    }

    /// Returns the bounding box of the sphere.
    #[inline(always)]
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
