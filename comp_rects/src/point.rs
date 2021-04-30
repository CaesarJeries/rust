use std::fmt;

extern crate approx;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {

    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x, y, z: 0 as f64
        }
    }

    pub fn origin() -> Point {
        Point {
            x: 0.,
            y: 0.,
            z: 0.
        }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let d1 = (self.x - other.x).abs();
        let d2 = (self.y - other.y).abs();
        let d3 = (self.z - other.z).abs();
        
        (d1*d1 + d2*d2 + d3*d3).sqrt()
    }
}

impl fmt::Display for Point {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3})", self.x, self.y)        
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx::AbsDiffEq::abs_diff_eq(&self.x, &other.x, std::f64::EPSILON) &&
        approx::AbsDiffEq::abs_diff_eq(&self.y, &other.y, std::f64::EPSILON)
    }
}
impl Eq for Point {}