use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::{material::*, rtweekend};
use crate::onb::Onb;
use crate::ray::{self, Ray};
use crate::rtweekend::{INFINITY, PI, random_double};
use crate::vec3::{Point3, Vec3};

use std::rc::Rc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    // Stationary Sphere
    pub fn new_static_sphere(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::new_from_points(static_center - rvec, static_center + rvec);

        Sphere {
            center: Ray::new_no_time(static_center, Vec3::default()),
            radius: radius.max(0.0),
            mat: mat,
            bbox,
        }
    }

    // Moving Sphere
    pub fn new_dynamic_sphere(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        let center = Ray::new_no_time(center1, center2 - center1);
        let radius = radius.max(0.0);
        let mat = mat;

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
    pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }

    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2*((1.0-radius*radius/distance_squared).sqrt() - 1.0);

        let phi = 2.0* rtweekend::PI *r1;
        let x = (phi).cos() * (1.0-z*z).sqrt();
        let y = (phi).sin() * (1.0-z*z).sqrt();

        return Vec3::new(x, y, z);
    }

}

impl Hittable for Sphere {
    //color map: n is a unit length => x, y, z E (-1.0, 1.0) ==> (0.0, 1.0) => (red, green, blue)

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

        rec.mat = Some(Rc::clone(&self.mat));

        return true;
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        // This method only works for stationary spheres.

        let mut rec = HitRecord::default();
        if !self.hit(&Ray::new_no_time(*origin, *direction), Interval::new(0.001, INFINITY), &mut rec){
            return 0.0;
        }
        let dist_squared = (self.center.at(0.0) - *origin).length_squared();
        let cos_theta_max = (1.0 - self.radius * self.radius/ dist_squared).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        return  1.0 / solid_angle;
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center.at(0.0) - *origin;
        let distance_squared = direction.length_squared();
        let uvw = Onb::new(&direction);
        return uvw.transform(&Sphere::random_to_sphere(self.radius, distance_squared));
    }

}
