// bvh.rs — parallel BVH builder (requires project changes listed below)

use std::sync::Arc;
use std::cmp::Ordering;


use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::Ray;

const PARALLEL_THRESHOLD: usize = 64; // tune this for your CPU / scenes

pub struct BvhNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new_from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        if len == 0 {
            panic!("Cannot create BVH from empty list");
        }
        // Note: HittableList.objects must be Vec<Arc<dyn Hittable + Send + Sync>>
        BvhNode::new(&mut list.objects[..], 0, len)
    }

    /// Build a BVH over the range [start, end) of `objects`.
    /// objects: mut slice of Arc<dyn Hittable + Send + Sync>, so we can reorder.
    pub fn new(objects: &mut [Arc<dyn Hittable + Send + Sync>], start: usize, end: usize) -> Self {
        // Build the bounding box of the span of source objects.
        let mut bbox = AABB::EMPTY;
        for object_index in start..end {
            bbox = AABB::new_(bbox, objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();
        let object_span = end - start;

        // Leaf cases
        if object_span == 1 {
            let leaf = Arc::clone(&objects[start]);
            return Self { left: leaf.clone(), right: leaf, bbox };
        }
        if object_span == 2 {
            let left = Arc::clone(&objects[start]);
            let right = Arc::clone(&objects[start + 1]);
            return Self { left, right, bbox };
        }

        // For object_span >= 3
        // Create (Arc, AABB) pairs for the range and sort them by AABB.min on selected axis.
        let mut pairs: Vec<(Arc<dyn Hittable + Send + Sync>, AABB)> = objects[start..end]
            .iter()
            .map(|o| (Arc::clone(o), o.bounding_box()))
            .collect();

        pairs.sort_by(|a, b| {
            let amin = a.1.axis_interval(axis).min;
            let bmin = b.1.axis_interval(axis).min;
            amin.partial_cmp(&bmin).unwrap_or(Ordering::Equal)
        });

        // Write back sorted Arc handles into objects[start..end]
        for (i, (obj, _b)) in pairs.into_iter().enumerate() {
            objects[start + i] = obj;
        }

        let mid = start + object_span / 2;

        // Build children — parallelize recursion when the span is large.
        if object_span > PARALLEL_THRESHOLD {
            let (left_slice, right_slice) = objects.split_at_mut(mid);
            let (left_node, right_node) = rayon::join(
                || BvhNode::new(left_slice, 0, left_slice.len()),
                || BvhNode::new(right_slice, 0, right_slice.len()),
            );
            Self {
                left: Arc::new(left_node),
                right: Arc::new(right_node),
                bbox,
            }
        } else {
            let left_node = Arc::new(BvhNode::new(objects, start, mid));
            let right_node = Arc::new(BvhNode::new(objects, mid, end));
            Self {
                left: left_node,
                right: right_node,
                bbox,
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Fast reject with node bbox
        if !self.bbox.hit(r, ray_t, rec) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);

        let arg = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, arg), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
