use crate::vec3::{Point3, Vec3};

/// A ray with origin, direction, and time parameter.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    org: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    /// Creates a new ray with origin, direction, and time.
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            org: origin,
            dir: direction,
            tm: time,
        }
    }

    /// Creates a new ray with origin and direction; time is zero.
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

// Getters for ray components
impl Ray {
    pub fn origin(&self) -> Point3 {
        self.org
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}

// Utility methods
impl Ray {
    /// Returns the point along the ray at parameter `t`.
    #[inline(always)]
    pub fn at(&self, t: f64) -> Point3 {
        self.org + t * self.dir // P(t) = origin + t * direction
    }
}
