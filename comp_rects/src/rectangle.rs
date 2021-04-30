extern crate draw;

use std::fmt;
use std::collections::HashMap;
use std::vec::Vec;

use draw::Canvas;
use draw::Color;
use draw::Drawing;
use draw::Shape;
use draw::Style;

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
    
    pub fn draw(&self) {
        let mut canvas = Canvas::new(100, 100);

        let rect = Drawing::new()
                    .with_shape(Shape::Rectangle {
                        width: (self.top_right.x - self.bottom_left.x) as u32,
                        height: (self.top_right.y - self.bottom_left.y) as u32
                    })
                    .with_xy(25.0, 25.0)
                    .with_style(Style::stroked(5, Color::black()));
        canvas.display_list.add(rect);
        draw::render::save(
                    &canvas,
                    "tests/svg/basic_end_to_end2.svg",
                    draw::SvgRenderer::new(),
                    )
                    .expect("Failed to save")
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|bottom left: {}, top right: {}|", self.bottom_left, self.top_right)
    }
}

