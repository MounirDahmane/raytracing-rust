use crate::Ray;
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use std::sync::Arc;

/// Bounding Volume Hierarchy (BVH) node for efficient ray intersection tests.
/// Each node contains left and right child hittables and a bounding box enclosing them.
pub struct BvhNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: Aabb,
}

impl BvhNode {
    /// Constructs a BVH node from a mutable reference to a hittable list.
    /// Panics if the list is empty.
    pub fn new_from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        if len == 0 {
            panic!("Cannot create BVH from empty list");
        }
        BvhNode::new(&mut list.objects[..], 0, len)
    }

    /// Recursively builds a BVH node from a slice of hittables within [start, end).
    pub fn new(objects: &mut [Arc<dyn Hittable + Send + Sync>], start: usize, end: usize) -> Self {
        // Compute bounding box enclosing all objects in this range.
        let mut bbox = Aabb::EMPTY;

        for item in objects.iter().take(end).skip(start) {
            bbox = Aabb::new_(bbox, item.bounding_box());
        }

        let axis = bbox.longest_axis();
        let object_span = end - start;

        let left: Arc<dyn Hittable + Send + Sync>;
        let right: Arc<dyn Hittable + Send + Sync>;

        if object_span == 1 {
            // Leaf node: both children point to the single object.
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            // Leaf node with two objects.
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            // Sort objects by bounding box minimum on the chosen axis.
            objects[start..end].sort_by(|a, b| {
                a.bounding_box()
                    .axis_interval(axis)
                    .min
                    .partial_cmp(&b.bounding_box().axis_interval(axis).min)
                    .unwrap()
            });

            let mid = start + object_span / 2;

            left = Arc::new(BvhNode::new(objects, start, mid));
            right = Arc::new(BvhNode::new(objects, mid, end));
        }

        Self { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    /// Checks whether the ray hits this BVH node by first testing bounding box, then children.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t, rec) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);

        let arg = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, arg), rec);

        hit_left || hit_right
    }

    /// Returns the bounding box enclosing this BVH node.
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
