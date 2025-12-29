use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list;
use crate::{ray::Ray, interval::Interval};
use crate::aabb::AABB;
use crate::Rc;
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

    fn bounding_box(&self) -> AABB { return self.bbox; }
}
