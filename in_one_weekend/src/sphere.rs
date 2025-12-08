use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::{self, Ray};
use crate::vec3::{Point3, Vec3};
use crate::material::*;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
            mat: mat,
        }
    }
}
impl Hittable for Sphere {
    //color map: n is a unit length => x, y, z E (-1.0, 1.0) ==> (0.0, 1.0) => (red, green, blue)

    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Some(Rc::clone(&self.mat));

        return true;
    }
}
