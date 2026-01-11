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

    /// Union of two intervals, covers both.
    pub fn new_(a: Interval, b: Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Interval { min, max }
    }

    /// Interval length.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Checks if x ∈ [min, max].
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if x ∈ (min, max).
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamp x to [min, max].
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    /// Empty interval (no range).
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    /// Entire real line interval.
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// Expand interval by delta, padding equally on both sides.
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

    /// Shift interval by displacement.
    fn add(self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    /// Allow displacement + interval.
    fn add(self, ival: Interval) -> Interval {
        ival + self
    }
}
