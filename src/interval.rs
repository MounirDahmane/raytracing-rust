use core::f64;
use std::ops::Add;

/// Represents a numeric interval [min, max].
#[derive(Copy, Clone)]
pub struct Interval {
    /// Minimum bound of the interval.
    pub min: f64,
    /// Maximum bound of the interval.
    pub max: f64,
}

impl Interval {
    /// Creates a new interval from min and max values.
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    /// Creates a new interval that encompasses both given intervals.
    pub fn new_(a: Interval, b: Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Interval { min, max }
    }

    /// Returns the size (length) of the interval.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Checks if a value is inside or on the boundary of the interval.
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if a value is strictly inside the interval (excluding boundaries).
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamps a value to the interval range.
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    /// An empty interval (min > max).
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    /// The universal interval covering all real numbers.
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// Expands the interval by `delta` equally on both sides.
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

    /// Adds a displacement to both bounds of the interval.
    fn add(self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    /// Adds a displacement to both bounds of the interval (reversed operands).
    fn add(self, ival: Interval) -> Interval {
        ival + self
    }
}
