use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    // The default AABB is empty, since intervals are empty by default.
    pub fn new_empty() -> Self {
        Self {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut x = x;
        let mut y = y;
        let mut z = z;

        AABB::pad_to_minimums(&mut x, &mut y, &mut z);

        Self { x, y, z }
    }

    pub fn new_from_points(a: Point3, b: Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.

        let mut x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };

        let mut y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };

        let mut z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        AABB::pad_to_minimums(&mut x, &mut y, &mut z);

        Self { x, y, z }
    }

    pub fn new_(box0: AABB, box1: AABB) -> Self {
        Self {
            x: Interval::new_(box0.x, box1.x),
            y: Interval::new_(box0.y, box1.y),
            z: Interval::new_(box0.z, box1.z),
        }
    }
}

impl AABB {
    pub fn longest_axis(&self) -> i8 {
        // Returns the index of the longest axis of the bounding box.
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0;
            } else {
                return 2;
            }
        } else {
            if self.y.size() > self.z.size() {
                return 1;
            } else {
                return 2;
            }
        }
    }
    pub fn axis_interval(&self, n: i8) -> &Interval {
        if n == 1 {
            return &self.y;
        }
        if n == 2 {
            return &self.z;
        }
        return &self.x;
    }

    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: AABB = AABB {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    fn pad_to_minimums(x: &mut Interval, y: &mut Interval, z: &mut Interval) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.

        let delta = 0.0001;

        if x.size() < delta {
            *x = x.expand(delta);
        }
        if y.size() < delta {
            *y = y.expand(delta);
        }
        if z.size() < delta {
            *z = z.expand(delta);
        }
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, offset: Vec3) -> AABB {
        AABB::new(
            self.x + offset.x(),
            self.y + offset.y(),
            self.z + offset.z(),
        )
    }
}

impl Add<AABB> for Vec3 {
    type Output = AABB;

    fn add(self, bbox: AABB) -> AABB {
        bbox + self
    }
}



impl Hittable for AABB {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let ray_orig: Point3 = r.origin();
        let ray_dir = r.direction();

        let mut ray_t = ray_t;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis as usize];

            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0
                };
                if t1 < ray_t.max {
                    ray_t.max = t1
                };
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1
                };
                if t0 < ray_t.max {
                    ray_t.max = t0
                };
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;
    }

    fn bounding_box(&self) -> AABB {
        AABB::new_empty()
    }
}
