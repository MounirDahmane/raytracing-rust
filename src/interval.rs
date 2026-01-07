use core::f64;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    /// Create a new interval covering both a and b (union).
    pub fn new_(a: Interval, b: Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Interval { min, max }
    }

    /// Length of the interval.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Checks if x is within [min, max].
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if x is strictly inside (min, max).
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamp x to the interval boundaries.
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    /// Interval representing an empty range.
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    /// Interval representing the entire real line.
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// Expand the interval by delta (adds padding equally on both sides).
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::UNIVERSE
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    /// Shift the interval by a displacement value.
    fn add(self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    /// Allow adding Interval to f64 (commutative).
    fn add(self, ival: Interval) -> Interval {
        ival + self
    }
}
