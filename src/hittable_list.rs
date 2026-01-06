use crate::hittable::{HitRecord, Hittable};
use crate::{interval::Interval, ray::Ray};

/// A list of hittable objects, representing a scene or group of objects.
pub struct HittableList {
    /// Collection of objects implementing the Hittable trait.
    pub objects: Vec<Box<dyn Hittable + Sync>>,
}

impl HittableList {
    /// Creates an empty `HittableList`.
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    /// Adds a single hittable object to the list.
    pub fn new_list(&mut self, object: Box<dyn Hittable + Sync>) {
        self.add(object)
    }

    /// Clears all objects from the list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Adds a hittable object to the list.
    pub fn add(&mut self, object: Box<dyn Hittable + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    /// Checks for ray intersections against all objects, returning true if any hit occurs.
    /// Updates `rec` with the closest hit record.
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
}
