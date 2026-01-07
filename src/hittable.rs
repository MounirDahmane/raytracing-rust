use std::sync::Arc;
use crate::aabb::AABB;
use crate::material::Material;
use crate::rtweekend::{self, INFINITY};
use crate::vec3::{Point3, Vec3};
use crate::{interval::Interval, Ray};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material + Send + Sync>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the normal to always point against the incoming ray direction
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: None,
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}

pub struct Translate {
    object: Arc<dyn Hittable + Send + Sync>, 
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        // Compute bounding box after translation by offset
        let bbox = object.bounding_box() + offset;
        Self { object, offset, bbox }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Shift ray backwards by offset to test intersection in original space
        let offset_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move intersection point forward by offset to world space
        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {
        let radians = rtweekend::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        // Rotate all 8 bounding box corners and find new AABB
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.x.max + ((1 - i) as f64) * bbox.x.min;
                    let y = (j as f64) * bbox.y.max + ((1 - j) as f64) * bbox.y.min;
                    let z = (k as f64) * bbox.z.max + ((1 - k) as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::new_from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Rotate ray origin and direction to object's local coordinate system
        let origin = Point3::new(
            self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z(),
            r.origin().y(),
            self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z(),
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z(),
            r.direction().y(),
            self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z(),
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // Rotate intersection point and normal back to world coordinate system
        rec.p = Point3::new(
            self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z(),
            rec.p.y(),
            -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z(),
        );

        rec.normal = Vec3::new(
            self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
            rec.normal.y(),
            -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
        );

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
