use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, PI, random_double};
use crate::vec3::{Point3, Vec3};
use crate::{material::*, rtweekend};

use std::sync::Arc;

/// A sphere that can be stationary or moving, with an associated material.
pub struct Sphere {
    center: Ray, // Represents the center and velocity (for moving sphere)
    radius: f64,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: Aabb,
}

impl Sphere {
    /// Creates a stationary sphere with a fixed center.
    pub fn new_static_sphere(
        static_center: Point3,
        radius: f64,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::new_from_points(static_center - rvec, static_center + rvec);

        Sphere {
            center: Ray::new_no_time(static_center, Vec3::default()),
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }

    /// Creates a moving sphere from `center1` at time 0 to `center2` at time 1.
    pub fn new_dynamic_sphere(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let center = Ray::new_no_time(center1, center2 - center1);
        let radius = radius.max(0.0);

        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = Aabb::new_from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        let bbox = Aabb::new_(box1, box2);

        Sphere {
            center,
            radius,
            mat,
            bbox,
        }
    }
}

impl Sphere {
    /// Computes UV coordinates for a point on the unit sphere.
    ///
    /// `p` should be a point on the sphere surface (radius 1, centered at origin).
    ///
    /// `u` is the angle around the Y axis from X=-1, in [0,1].
    ///
    /// `v` is the angle from Y=-1 to Y=+1, in [0,1].
    pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }

    /// Generates a random vector within a sphere defined by radius and squared distance.
    ///
    /// Used for importance sampling towards the sphere surface.
    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * rtweekend::PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vec3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    /// Checks if a ray hits the sphere between the given interval.
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

        // Find the nearest root that lies in the acceptable range.
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
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    /// Returns the PDF value for sampling rays towards the sphere.
    ///
    /// Only works for stationary spheres.
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new_no_time(*origin, *direction),
            Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }
        let dist_squared = (self.center.at(0.0) - *origin).length_squared();
        let cos_theta_max = (1.0 - self.radius * self.radius / dist_squared).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    /// Generates a random direction towards the sphere from a given origin.
    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center.at(0.0) - *origin;
        let distance_squared = direction.length_squared();
        let uvw = Onb::new(&direction);
        uvw.transform(&Sphere::random_to_sphere(self.radius, distance_squared))
    }
}
