use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list;
use crate::Rc;
use crate::rtweekend::random_int_range;
use crate::{interval::Interval, ray::Ray};
use crate::vec3::*;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    pub bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::new_empty(),
        }
    }
    pub fn new_list(&mut self, object: Rc<dyn Hittable>) {
        self.add(object)
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        let object_bbox = object.bounding_box();
        self.objects.push(object);
        self.bbox = AABB::new_(self.bbox, object_bbox);
    }

}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                std::mem::swap(rec, &mut temp_rec);
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }


    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }
        return sum;
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let int_size = self.objects.len() as i32;

        let index = random_int_range(0, int_size - 1) as usize;
        self.objects[index].random(origin)
    }
}
