use rand::{rng, Rng};

/// Mathematical constant for infinity.
pub const INFINITY: f64 = f64::INFINITY;
/// Mathematical constant for π.
pub const PI: f64 = 3.1415926535897932385;

/// Converts degrees to radians.
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Generates a random double in the range [0, 1).
#[inline]
pub fn random_double() -> f64 {
    rng().random::<f64>()
}

/// Generates a random double in the range [min, max).
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

/// Generates a random integer in the inclusive range [min, max].
#[inline]
pub fn random_int_range(min: i32, max: i32) -> i32 {
    // Add 1 to max to make the range inclusive
    let max_exclusive = max + 1;
    random_double_range(min as f64, max_exclusive as f64) as i32
}
