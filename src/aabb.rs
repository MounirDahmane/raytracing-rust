use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    /// Creates an empty bounding box with empty intervals.
    pub fn new_empty() -> Self {
        Self {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }

    /// Creates a bounding box from intervals, padding any that are too narrow.
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut x = x;
        let mut y = y;
        let mut z = z;

        Aabb::pad_to_minimums(&mut x, &mut y, &mut z);

        Self { x, y, z }
    }

    /// Creates an Aabb covering two points.
    pub fn new_from_points(a: Point3, b: Point3) -> Self {
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

        Aabb::pad_to_minimums(&mut x, &mut y, &mut z);

        Self { x, y, z }
    }

    /// Creates an Aabb that encloses two bounding boxes.
    pub fn new_(box0: Aabb, box1: Aabb) -> Self {
        Self {
            x: Interval::new_(box0.x, box1.x),
            y: Interval::new_(box0.y, box1.y),
            z: Interval::new_(box0.z, box1.z),
        }
    }

    /// Returns the index of the longest axis: 0=x, 1=y, 2=z.
    pub fn longest_axis(&self) -> i8 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    /// Returns a reference to the interval of the specified axis.
    pub fn axis_interval(&self, n: i8) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub const EMPTY: Aabb = Aabb {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Aabb = Aabb {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    /// Ensures intervals have a minimum size to avoid degenerate bounding boxes.
    fn pad_to_minimums(x: &mut Interval, y: &mut Interval, z: &mut Interval) {
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

// Translate Aabb by a vector offset.
impl Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, offset: Vec3) -> Aabb {
        Aabb::new(
            self.x + offset.x(),
            self.y + offset.y(),
            self.z + offset.z(),
        )
    }
}

// Delegate adding an Aabb to a Vec3 to Vec3 + Aabb.
impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, bbox: Aabb) -> Aabb {
        bbox + self
    }
}

impl Hittable for Aabb {
    /// Checks if the ray hits the bounding box within the interval ray_t.
    fn hit(&self, r: &Ray, mut ray_t: Interval, _rec: &mut HitRecord) -> bool {
        let ray_orig: Point3 = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis as usize];

            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    /// Returns the bounding box of this object (itself).
    fn bounding_box(&self) -> Aabb {
        *self
    }
}
