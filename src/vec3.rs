use crate::rtweekend;
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// A 3D vector with `f64` components.
///
/// This type is used for both geometric vectors and points in 3D space.
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub vector: [f64; 3],
}

/// A point in 3D space.
///
/// This is a semantic alias of `Vec3` used to distinguish positions from directions.
pub type Point3 = Vec3;

//
// Constructors
//
impl Vec3 {
    /// Creates a vector from a `(x, y, z)` tuple.
    pub fn new_from_tuple(tuple: (f64, f64, f64)) -> Self {
        Self {
            vector: [tuple.0, tuple.1, tuple.2],
        }
    }

    /// Creates a vector from explicit `x`, `y`, and `z` components.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { vector: [x, y, z] }
    }
}

impl Default for Vec3 {
    /// Returns the zero vector `(0, 0, 0)`.
    fn default() -> Self {
        Vec3 {
            vector: [0.0, 0.0, 0.0],
        }
    }
}

//
// Component accessors
//
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

//
// Vector operations
//
impl Vec3 {
    /// Returns the squared Euclidean length of the vector.
    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.vector[0] * self.vector[0]
            + self.vector[1] * self.vector[1]
            + self.vector[2] * self.vector[2]
    }

    /// Returns the Euclidean length of the vector.
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Computes the dot product between two vectors.
    #[inline]
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.vector[0] * other.vector[0]
            + self.vector[1] * other.vector[1]
            + self.vector[2] * other.vector[2]
    }

    /// Computes the cross product between two vectors.
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            vector: [
                self.vector[1] * other.vector[2] - self.vector[2] * other.vector[1],
                self.vector[2] * other.vector[0] - self.vector[0] * other.vector[2],
                self.vector[0] * other.vector[1] - self.vector[1] * other.vector[0],
            ],
        }
    }

    /// Returns a unit-length version of the given vector.
    ///
    /// If the input vector is zero, the zero vector is returned.
    #[inline]
    pub fn unit_vector(v: Vec3) -> Vec3 {
        if v.length() == 0.0 {
            Vec3::default()
        } else {
            v / v.length()
        }
    }

    /// Returns a random vector with each component in `[0, 1)`.
    pub fn random() -> Self {
        Self::new(
            rtweekend::random_double(),
            rtweekend::random_double(),
            rtweekend::random_double(),
        )
    }

    /// Returns a random vector with each component in `[min, max)`.
    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            rtweekend::random_double_range(min, max),
            rtweekend::random_double_range(min, max),
            rtweekend::random_double_range(min, max),
        )
    }

    /// Returns a uniformly distributed random unit vector using rejection sampling.
    #[inline]
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();

            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    /// Returns a cosine-weighted random direction in the hemisphere aligned with +Z.
    #[inline]
    pub fn random_cosine_direction() -> Self {
        let r1 = rtweekend::random_double();
        let r2 = rtweekend::random_double();

        let phi = 2.0 * rtweekend::PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        let z = (1.0 - r2).sqrt();

        Vec3::new(x, y, z)
    }

    /// Returns a random point inside the unit disk in the XY plane.
    #[inline]
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                rtweekend::random_double_range(-1.0, 1.0),
                rtweekend::random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Returns a random unit vector constrained to the hemisphere defined by `normal`.
    #[inline]
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    /// Returns `true` if the vector is close to zero in all dimensions.
    #[inline]
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.vector[0] < s && self.vector[1] < s && self.vector[2] < s
    }

    /// Reflects a vector about a normal.
    #[inline]
    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        *v - 2.0 * v.dot(n) * *n
    }

    /// Computes the refracted ray according to Snell's law.
    #[inline]
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-uv.dot(n)).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;
        r_out_perp + r_out_parallel
    }
}

//
// Formatting
//
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

//
// Arithmetic operators
//
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

//
// Indexing
//
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

//
// Equality
//
impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.vector[0] == other.vector[0]
            && self.vector[1] == other.vector[1]
            && self.vector[2] == other.vector[2]
    }
}
