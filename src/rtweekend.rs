use rand::{Rng, rng};

/// Represents positive infinity for `f64`.
pub const INFINITY: f64 = f64::INFINITY;

/// Pi constant for floating-point calculations.
pub const PI: f64 = 3.1415926535897932385;

/// Converts degrees to radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
/// Returns a random floating-point number in the range [0, 1).
pub fn random_double() -> f64 {
    rng().random::<f64>()
}

/// Returns a random floating-point number in the range [min, max).
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

#[inline]
/// Returns a random integer in the inclusive range [min, max].
pub fn random_int_range(min: i32, max: i32) -> i32 {
    let max = max + 1;
    random_double_range(min as f64, max as f64) as i32
}
