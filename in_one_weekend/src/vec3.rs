use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use std::vec;
use crate::rtweekend;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub vector: [f64; 3],
}

pub type Point3 = Vec3;
//constructors
impl Vec3 {
    pub fn new_from_tuple(tuple: (f64, f64, f64)) -> Self {
        Self {
            vector: [tuple.0, tuple.1, tuple.2],
        }
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { vector: [x, y, z] }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            vector: [0.0, 0.0, 0.0],
        }
    }
}

//position
impl Vec3 {
    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.vector[0]
    }

    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.vector[1]
    }

    #[inline(always)]
    pub fn z(&self) -> f64 {
        self.vector[2]
    }
}

//utilitis
impl Vec3 {
    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.vector[0] * self.vector[0]
            + self.vector[1] * self.vector[1]
            + self.vector[2] * self.vector[2]
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.vector[0] * other.vector[0]
            + self.vector[1] * other.vector[1]
            + self.vector[2] * other.vector[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            vector: [
                self.vector[1] * other.vector[2] - self.vector[2] * other.vector[1],
                self.vector[2] * other.vector[0] - self.vector[0] * other.vector[2],
                self.vector[0] * other.vector[1] - self.vector[1] * other.vector[0],
            ],
        }
    }

    #[inline]
    pub fn unit_vector(v: Vec3) -> Vec3 {
        if v.length() == 0.0 {
            return Vec3::default();
        } else {
            v / v.length()
        }
    }
    pub fn random() -> Self {
        Self::new(rtweekend::random_double(), rtweekend::random_double(), rtweekend::random_double())
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(rtweekend::random_double_range(min, max), rtweekend::random_double_range(min, max), rtweekend::random_double_range(min, max))
    }

    #[inline]
    // rejection method
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }
    #[inline]
    pub fn random_on_hemisphere(normal: &Vec3) -> Self{
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            return  on_unit_sphere;
        }
        else {
            return  -on_unit_sphere;
        }
    }
}
    

//operations

impl Display for Vec3 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.vector[0], self.vector[1], self.vector[2]
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            vector: [
                self.vector[0] + other.vector[0],
                self.vector[1] + other.vector[1],
                self.vector[2] + other.vector[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.vector[0] += rhs.vector[0];
        self.vector[1] += rhs.vector[1];
        self.vector[2] += rhs.vector[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            vector: [
                self.vector[0] - other.vector[0],
                self.vector[1] - other.vector[1],
                self.vector[2] - other.vector[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.vector[0] -= rhs.vector[0];
        self.vector[1] -= rhs.vector[1];
        self.vector[2] -= rhs.vector[2];
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            vector: [-self.vector[0], -self.vector[1], -self.vector[2]],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Vec3 {
        Vec3 {
            vector: [
                self.vector[0] * rhs.vector[0],
                self.vector[1] * rhs.vector[1],
                self.vector[2] * rhs.vector[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            vector: [self.vector[0] * t, self.vector[1] * t, self.vector[2] * t],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            vector: [v.vector[0] * self, v.vector[1] * self, v.vector[2] * self],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.vector[0] *= t;
        self.vector[1] *= t;
        self.vector[2] *= t;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        if t == 0.0 {
            panic!("Attempt to divide Vec3 by zero");
        }
        self * (1.0 / t)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            panic!("Attempt to divide Vec3 by zero");
        }
        self.vector[0] /= rhs;
        self.vector[1] /= rhs;
        self.vector[2] /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.vector[0],
            1 => &self.vector[1],
            2 => &self.vector[2],
            _ => panic!("Index out of range for Vec3"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.vector[0],
            1 => &mut self.vector[1],
            2 => &mut self.vector[2],
            _ => panic!("Index out of range for Vec3"),
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.vector[0] == other.vector[0]
            && self.vector[1] == other.vector[1]
            && self.vector[2] == other.vector[2]
    }
}
