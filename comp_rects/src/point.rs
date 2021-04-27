use std::fmt;

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {

    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            x, y, z
        }
    }
}

impl fmt::Display for Point {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)        
    }
}
