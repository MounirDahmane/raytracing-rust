// class aabb {
//   public:
//     interval x, y, z;

//     aabb() {} // The default AABB is empty, since intervals are empty by default.

//     aabb(const interval& x, const interval& y, const interval& z)
//       : x(x), y(y), z(z) {}

//     aabb(const point3& a, const point3& b) {
//         // Treat the two points a and b as extrema for the bounding box, so we don't require a
//         // particular minimum/maximum coordinate order.

//         x = (a[0] <= b[0]) ? interval(a[0], b[0]) : interval(b[0], a[0]);
//         y = (a[1] <= b[1]) ? interval(a[1], b[1]) : interval(b[1], a[1]);
//         z = (a[2] <= b[2]) ? interval(a[2], b[2]) : interval(b[2], a[2]);
//     }

//     const interval& axis_interval(int n) const {
//         if (n == 1) return y;
//         if (n == 2) return z;
//         return x;
//     }

//     bool hit(const ray& r, interval ray_t) const {
//         const point3& ray_orig = r.origin();
//         const vec3&   ray_dir  = r.direction();

//         for (int axis = 0; axis < 3; axis++) {
//             const interval& ax = axis_interval(axis);
//             const double adinv = 1.0 / ray_dir[axis];

//             auto t0 = (ax.min - ray_orig[axis]) * adinv;
//             auto t1 = (ax.max - ray_orig[axis]) * adinv;

//             if (t0 < t1) {
//                 if (t0 > ray_t.min) ray_t.min = t0;
//                 if (t1 < ray_t.max) ray_t.max = t1;
//             } else {
//                 if (t1 > ray_t.min) ray_t.min = t1;
//                 if (t0 < ray_t.max) ray_t.max = t0;
//             }

//             if (ray_t.max <= ray_t.min)
//                 return false;
//         }
//         return true;
//     }
// };

use crate::interval::Interval;
use crate::vec3::Point3;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct AABB{
    x: Interval,
    y: Interval,
    z: Interval,
}
impl AABB {
    // The default AABB is empty, since intervals are empty by default.
    pub fn new_empty() -> Self {
        Self { x: Interval::default(), y: Interval::default(), z: Interval::default() }
    }
    
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z}
    }

    pub fn new_from_points(a: Point3, b: Point3) -> Self {

        let x = if a[0] <= b[0] {Interval::new(a[0], b[0])} 
                        else {Interval::new(b[0], a[0])};

        let y = if a[1] <= b[1] {Interval::new(a[1], b[1])} 
                        else {Interval::new(b[1], a[1])};

        let z = if a[2] <= b[2] {Interval::new(a[2], b[2])} 
                        else {Interval::new(b[2], a[2])};
        
        Self { x, y, z }
    }
}

impl AABB {
    
    pub fn axis_interval(&self, n: i32) -> &Interval {
        if n == 1 {return &self.y;}
        if n == 2 {return &self.z;}
        return &self.x;        
    }
}

impl Hittable for AABB {
    
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool{
        let ray_orig: Point3 = r.origin();
        let ray_dir = r.direction();
        
        let mut ray_t = ray_t;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis as usize];

            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {ray_t.min = t0};
                if t1 < ray_t.max {ray_t.max = t1};
            } 
            else {
                if t1 > ray_t.min {ray_t.min = t1};
                if t0 < ray_t.max {ray_t.max = t0};
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;
    }
}