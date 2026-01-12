use std::rc::Rc;

use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rtweekend;
use crate::rtweekend::random_double;
use crate::vec3::{Point3, Vec3};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct SpherePdf;

impl SpherePdf {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * rtweekend::PI)
    }
    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = Vec3::dot(&Vec3::unit_vector(*direction), &self.uvw.w());
        0.0_f64.max(cosine_theta / rtweekend::PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&Vec3::random_cosine_direction())
    }
}

pub struct HittablePdf {
    objects: Rc<dyn Hittable>,
    origin: Point3,
}

impl HittablePdf {
    pub fn new(objects: Rc<dyn Hittable>, origin: &Point3) -> Self {
        Self {
            objects,
            origin: *origin,
        }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

pub struct MixturePdf {
    p: [Rc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(p0: Rc<dyn Pdf>, p1: Rc<dyn Pdf>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
