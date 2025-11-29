use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    org: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            org: origin,
            dir: direction,
        }
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            org: Point3::default(),
            dir: Vec3::default(),
        }
    }
}

//getters
impl Ray {
    pub fn origin(&self) -> Point3 {
        self.org
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
}

//utils
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.org + t * self.dir // P(t) = origin + t * direction
    }
}
