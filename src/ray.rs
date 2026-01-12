use crate::vec3::Point3;
use crate::vec3::Vec3;

/// Represents a ray in 3D space, with an origin, direction, and optional time value.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    org: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    /// Creates a new ray with given origin, direction, and time.
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray {
            org: origin,
            dir: direction,
            tm: time,
        }
    }

    /// Creates a new ray with given origin and direction, with time set to 0.
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
            tm: f64::default(),
        }
    }
}

// Getters for the ray's components
impl Ray {
    /// Returns the origin point of the ray.
    pub fn origin(&self) -> Point3 {
        self.org
    }

    /// Returns the direction vector of the ray.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Returns the time associated with the ray.
    pub fn time(&self) -> f64 {
        self.tm
    }
}

// Utility methods for ray calculations
impl Ray {
    /// Returns the point along the ray at parameter `t`: P(t) = origin + t * direction
    pub fn at(&self, t: f64) -> Point3 {
        self.org + t * self.dir
    }
}
