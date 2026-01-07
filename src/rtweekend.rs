use rand::{rng, Rng};

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_double() -> f64 {
    rng().random::<f64>() // generates a float in [0,1)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

#[inline]
pub fn random_int_range(min: i32, max: i32) -> i32 {
    // Inclusive range [min, max]
    let max_exclusive = max + 1;
    random_double_range(min as f64, max_exclusive as f64) as i32
}
