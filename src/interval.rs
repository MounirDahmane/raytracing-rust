use core::f64;

pub struct Interval {
    /// Inclusive lower bound.
    pub min: f64,
    /// Inclusive upper bound.
    pub max: f64,
}

impl Interval {
    /// Creates a new interval [min, max].
    #[inline]
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    /// Returns the size (length) of the interval.
    #[inline]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Returns true if x is in [min, max].
    #[inline]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Returns true if x is strictly inside (min, max).
    #[inline]
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamps x to the interval bounds.
    #[inline]
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    /// Represents an empty interval.
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    /// Represents the entire range of all possible values.
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
}

impl Default for Interval {
    #[inline]
    fn default() -> Self {
        Interval::UNIVERSE
    }
}
