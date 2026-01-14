use rayon::prelude::*;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::{interval::Interval, ray::Ray};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>, // List of hittable objects
    pub bbox: Aabb,                                    // Bounding box encompassing all objects
}

impl HittableList {
    /// Creates a new empty hittable list.
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: Aabb::new_empty(),
        }
    }

    /// Adds an object and updates the bounding box accordingly.
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        let object_bbox = object.bounding_box();
        self.objects.push(object);
        self.bbox = Aabb::new_(self.bbox, object_bbox);
    }

    /// Clears the hittable list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Perform parallel intersection tests on all objects
        let hits = self
            .objects
            .par_iter()
            .filter_map(|object| {
                let mut temp_rec = HitRecord::default();
                if object.hit(r, ray_t, &mut temp_rec) {
                    Some(temp_rec)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Find the closest hit point among all hits
        if let Some(closest_hit) = hits
            .into_iter()
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
        {
            *rec = closest_hit;
            true
        } else {
            false
        }
    }

    /// Returns the bounding box enclosing all objects in the list.
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
