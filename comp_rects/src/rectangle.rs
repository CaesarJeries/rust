
use std::fmt;
use crate::point::Point;

pub struct Rect {
    bottom_left: Point,
    top_right: Point
}

impl Rect {
    pub fn new(bottom_left: Point, top_right: Point) -> Rect {
        Rect {
            bottom_left,
            top_right
        }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|bottom left: {}, top right: {}|", self.bottom_left, self.top_right)
    }
}

