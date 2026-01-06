use rand::{Rng, rng};

// Infinity constant
pub const INFINITY: f64 = f64::INFINITY;
// Pi constant
pub const PI: f64 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_double() -> f64 {
    rng().random::<f64>()
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
