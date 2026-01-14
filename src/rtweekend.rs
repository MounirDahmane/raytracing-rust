use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
}

#[inline]
pub fn random_double() -> f64 {
    RNG.with(|rng| rng.borrow_mut().gen_range(0.0..1.0))
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

#[inline]
pub fn random_int_range(min: i32, max: i32) -> i32 {
    let max = max + 1;
    random_double_range(min as f64, max as f64) as i32
}
