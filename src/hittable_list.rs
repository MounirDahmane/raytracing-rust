use std::sync::Arc;
use rayon::prelude::*;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::{interval::Interval, ray::Ray};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::new_empty(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        let object_bbox = object.bounding_box();
        self.objects.push(object);
        self.bbox = AABB::new_(self.bbox, object_bbox);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Parallel iteration over objects with Rayon
        let hits = self.objects.par_iter()
            .filter_map(|object| {
                let mut temp_rec = HitRecord::default();
                if object.hit(r, ray_t, &mut temp_rec) {
                    Some(temp_rec)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Find closest hit among hits collected in parallel
        if let Some(closest_hit) = hits.into_iter().min_by(|a, b| a.t.partial_cmp(&b.t).unwrap()) {
            *rec = closest_hit;
            true
        } else {
            false
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
