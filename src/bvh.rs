use rand::random;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::{self, HittableList};
use crate::interval::Interval;
use crate::Ray;
use crate::{rtweekend, Rc};

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new_from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        if len == 0 {
            panic!("Cannot create BVH from empty list");
        }
        BvhNode::new(&mut list.objects[..], 0, len)
    }
    pub fn new(objects: &mut [Rc<dyn Hittable>], start: usize, end: usize) -> Self {
        // Build the bounding box of the span of source objects.
        let mut bbox = AABB::EMPTY;

        for object_index in start..end {
            bbox = AABB::new_(bbox, objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

        let object_span = end - start;

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            // sort ONLY the range [start..end)
            objects[start..end].sort_by(|a, b| {
                a.bounding_box()
                    .axis_interval(axis)
                    .min
                    .partial_cmp(&b.bounding_box().axis_interval(axis).min)
                    .unwrap()
            });

            let mid = start + object_span / 2;

            left = Rc::new(BvhNode::new(objects, start, mid));
            right = Rc::new(BvhNode::new(objects, mid, end));
        }

        Self { left, right, bbox }
    }
}

impl BvhNode {}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t, rec) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);

        let arg = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, arg), rec);

        return hit_left || hit_right;
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}
