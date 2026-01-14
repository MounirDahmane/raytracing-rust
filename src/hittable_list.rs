use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::rtweekend::random_int_range;
use crate::vec3::*;
use crate::{interval::Interval, ray::Ray};
use std::sync::Arc;

/// A collection of hittable objects with a combined bounding box.
pub struct HittableList {
    /// The list of objects.
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    /// Bounding box enclosing all objects.
    pub bbox: Aabb,
}

impl HittableList {
    /// Creates a new, empty hittable list.
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: Aabb::new_empty(),
        }
    }

    /// Adds a single object to the list.
    pub fn new_list(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.add(object)
    }

    /// Clears all objects from the list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Adds an object to the list and updates the bounding box.
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        let object_bbox = object.bounding_box();
        self.objects.push(object);
        self.bbox = Aabb::new_(self.bbox, object_bbox);
    }
}

impl Hittable for HittableList {
    /// Tests if a ray hits any object in the list within the given interval.
    /// Records the closest hit information in `rec`.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
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

    /// Returns the bounding box enclosing all objects in the list.
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    /// Computes the PDF value for a given ray origin and direction by averaging over objects.
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }
        sum
    }

    /// Returns a random direction vector sampled from one of the objects in the list.
    fn random(&self, origin: &Point3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        let index = random_int_range(0, int_size - 1) as usize;
        self.objects[index].random(origin)
    }
}
