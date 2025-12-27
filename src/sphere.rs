use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::{self, Ray};
use crate::vec3::{Point3, Vec3};
use crate::material::*;
use std::rc::Rc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {

    // Stationary Sphere
    pub fn new_static_sphere(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center: Ray::new_no_time(static_center, Vec3::default()),
            radius: radius.max(0.0),
            mat: mat,
        }
    }
    // Moving Sphere
    pub fn new_dynamic_sphere(center1: Point3, center2: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center: Ray::new_no_time(center1, center2 - center1),
            radius: radius.max(0.0),
            mat: mat,
        }
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
        rec.mat = Some(Rc::clone(&self.mat));

        return true;
    }
}
