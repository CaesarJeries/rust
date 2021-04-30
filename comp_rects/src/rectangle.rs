
use std::fmt;
use std::collections::HashMap;
use std::vec::Vec;
use crate::point::Point;
use crate::line::line::Line;
use crate::line::line::HorizontalLine;
use crate::line::line::VerticalLine;

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

    pub fn get_extensions(&self, frame: &Rect) -> (HashMap<String, VerticalLine>, HashMap<String, HorizontalLine>) {
        let mut vextensions = HashMap::new();
        vextensions.insert("left".to_string(),
                           VerticalLine::new(
                               Point::new(self.bottom_left.x, frame.top_right.y),
                               Point::new(self.bottom_left.x, frame.bottom_left.y)
                            ));
        vextensions.insert("right".to_string(),
                          VerticalLine::new(
                            Point::new(self.top_right.x, frame.top_right.y),
                            Point::new(self.top_right.x, frame.bottom_left.y)
                          ));
       
        let mut hextensions = HashMap::new();
        hextensions.insert("top".to_string(),
                           HorizontalLine::new(
                               Point::new(frame.bottom_left.x, self.top_right.y),
                               Point::new(frame.top_right.x, self.top_right.y)
                           ));
        hextensions.insert("bottom".to_string(), HorizontalLine::new(
                               Point::new(frame.bottom_left.x, self.bottom_left.y),
                               Point::new(frame.top_right.x, self.bottom_left.y)
                           ));

        return (vextensions, hextensions);
    }

    pub fn get_intersection_with_frame(&self, frame: &Rect) -> Vec<Point> {
        let mut intersection_points = Vec::with_capacity(8);

        let (vlines, hlines) = self.get_extensions(frame);

        for (_, v) in vlines {
            for p in v.get_points() {
                intersection_points.push(p);
            }
        }
        
        for (_, v) in hlines {
            for p in v.get_points() {
                intersection_points.push(p);
            }
        }

        return intersection_points;
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|bottom left: {}, top right: {}|", self.bottom_left, self.top_right)
    }
}

