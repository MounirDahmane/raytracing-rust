use crate::vec3::{Point3, Vec3};

/// A ray in 3D space with an origin, direction, and an optional time parameter.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    org: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    /// Creates a new ray with the given origin, direction, and time.
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            org: origin,
            dir: direction,
            tm: time,
        }
    }

    /// Creates a new ray with origin and direction; time defaults to zero.
    pub fn new_no_time(origin: Point3, direction: Vec3) -> Self {
        Ray {
            org: origin,
            dir: direction,
            tm: 0.0,
        }
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            org: Point3::default(),
            dir: Vec3::default(),
            tm: 0.0,
        }
    }
}

impl Ray {
    /// Returns the origin point of the ray.
    pub fn origin(&self) -> Point3 {
        self.org
    }

    /// Returns the direction vector of the ray.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Returns the time parameter of the ray.
    pub fn time(&self) -> f64 {
        self.tm
    }

    /// Computes the point along the ray at parameter `t`.
    #[inline(always)]
    pub fn at(&self, t: f64) -> Point3 {
        self.org + t * self.dir
    }
}
