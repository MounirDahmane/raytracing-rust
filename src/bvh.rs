use std::cmp::Ordering;
use std::sync::Arc;

use crate::Ray;
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;

const PARALLEL_THRESHOLD: usize = 64; // Threshold for parallel recursion

pub struct BvhNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: Aabb,
}

impl BvhNode {
    /// Builds a BVH tree from a HittableList.
    pub fn new_from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        if len == 0 {
            panic!("Cannot create BVH from empty list");
        }
        // Build BVH over all objects
        BvhNode::new(&mut list.objects[..], 0, len)
    }

    /// Recursively builds a BVH over the slice [start, end) of objects.
    /// Objects are sorted along the longest axis of their combined bounding box.
    pub fn new(objects: &mut [Arc<dyn Hittable + Send + Sync>], start: usize, end: usize) -> Self {
        // Compute bounding box enclosing all objects in [start, end)
        let mut bbox = Aabb::EMPTY;
        for item in objects.iter().take(end).skip(start) {
            bbox = Aabb::new_(bbox, item.bounding_box());
        }

        let axis = bbox.longest_axis();
        let object_span = end - start;

        // Leaf node: single object
        if object_span == 1 {
            let leaf = Arc::clone(&objects[start]);
            return Self {
                left: leaf.clone(),
                right: leaf,
                bbox,
            };
        }

        // Leaf node: two objects
        if object_span == 2 {
            let left = Arc::clone(&objects[start]);
            let right = Arc::clone(&objects[start + 1]);
            return Self { left, right, bbox };
        }

        // For 3 or more objects, sort by bounding box minimum on longest axis
        let mut pairs: Vec<(Arc<dyn Hittable + Send + Sync>, Aabb)> = objects[start..end]
            .iter()
            .map(|o| (Arc::clone(o), o.bounding_box()))
            .collect();

        pairs.sort_by(|a, b| {
            let amin = a.1.axis_interval(axis).min;
            let bmin = b.1.axis_interval(axis).min;
            amin.partial_cmp(&bmin).unwrap_or(Ordering::Equal)
        });

        // Write back sorted objects
        for (i, (obj, _)) in pairs.into_iter().enumerate() {
            objects[start + i] = obj;
        }

        let mid = start + object_span / 2;

        // Build children nodes, using parallel recursion if large enough
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
    /// Returns true if the ray hits any object in this BVH node.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Quick reject if ray misses bounding box
        if !self.bbox.hit(r, ray_t, rec) {
            return false;
        }

        // Check left subtree
        let hit_left = self.left.hit(r, ray_t, rec);

        // Limit right subtree ray interval to nearest hit on left subtree to prune search
        let upper_bound = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self
            .right
            .hit(r, Interval::new(ray_t.min, upper_bound), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
