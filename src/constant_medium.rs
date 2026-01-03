use std::rc::Rc;

use crate::{aabb::AABB, Ray, hittable::{HitRecord, Hittable}, interval::Interval, material::Material, rtweekend::{self, INFINITY}, texture::Texture, vec3::Vec3};
use crate::material::Isotropic;
use crate::color::Color;

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {

    pub fn new(boundary: Rc<dyn Hittable>, density: f64, tex: Rc<dyn Texture>) -> Self {
        Self { boundary, neg_inv_density: -1.0 / density , phase_function: Rc::new(Isotropic::new_(tex)) }
    }

    pub fn new_(boundary: Rc<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self { boundary, neg_inv_density: -1.0 / density, phase_function: Rc::new(Isotropic::new(albedo)) }
    }

}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {

        let mut rec1= HitRecord::default();
        let mut rec2= HitRecord::default();

        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1){
            return false;
        }

        if !self.boundary.hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2){
            return false;
        }

        if rec1.t < ray_t.min { rec1.t = ray_t.min;}
        if rec2.t > ray_t.max { rec2.t = ray_t.max;}

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (rtweekend::random_double()).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0,0.0,0.0);  // arbitrary
        rec.front_face = true;     // also arbitrary
        rec.mat = Some(self.phase_function.clone());

        return true;
    }
    
    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}

