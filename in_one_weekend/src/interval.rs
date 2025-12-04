const interval interval::empty    = interval(+infinity, -infinity);
const interval interval::universe = interval(-infinity, +infinity);

pub struct interval {
    pub min : f64,
    pub max : f64, 
}

impl interval{
    interval() : min(+infinity), max(-infinity) {} // Default interval is empty
    pub fn interval(min: f64, max: f64) -> Self{
        interval { min, max }
    }
    pub fn size() -> f64 {
        max - min
    }
    pub fn contains(x: f64) -> bool{
        min <= x && x <= max
    }
    pub fn surrounds(x: f64) -> bool{
        min < x && x < max
    }
    interval() : min(+infinity), max(-infinity) {} // Default interval is empty
    static const interval empty, universe;
}