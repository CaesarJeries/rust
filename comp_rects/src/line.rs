extern crate approx;

pub mod line {


    use std::fmt;
    use std::cmp::PartialOrd;
    use std::vec::Vec;
    use crate::point::Point;

    pub trait Line {
        fn get_points(&self) -> Vec<Point>;
    }

    pub struct VerticalLine {
        top: Point,
        bottom: Point
    }

    pub struct HorizontalLine {
        left: Point,
        right: Point
    }

    impl VerticalLine {
        pub fn new(top: Point, bottom: Point) -> VerticalLine {
            if !(approx::AbsDiffEq::abs_diff_eq(&top.x, &bottom.x, std::f64::EPSILON)) {
                panic!("Line is not vertical");
            }

            let (top_y, bottom_y) = match PartialOrd::lt(&top.y, &bottom.y) {
                true => (bottom.y, top.y),
                false => (top.y, bottom.y)
            };

            VerticalLine {
                top: Point::new(top.x, top_y),
                bottom: Point::new(top.x, bottom_y)
            }
        }

        pub fn get_intersection(&self, hline: &HorizontalLine) -> Point {
            if (hline.left.y < self.bottom.y) | (hline.left.y > self.top.y) {
                println!("Horizontal line: {}", hline);
                println!("Vertical line: {}", self);
                panic!("Lines do not intersect");
            }

            if (self.bottom.x < hline.left.x) | (self.bottom.x > hline.right.x) {
                println!("Horizontal line: {}", hline);
                println!("Vertical line: {}", self);
                panic!("Lines do not intersect");
            }

            Point::new(self.bottom.x, hline.left.y)
        }
    }

    impl HorizontalLine {
        pub fn new(left: Point, right: Point) -> HorizontalLine {
            if !(approx::AbsDiffEq::abs_diff_eq(&left.y, &right.y, std::f64::EPSILON)) {
                panic!("Line is not horizontal");
            }

            let (left_x, right_x) = match PartialOrd::lt(&right.x, &left.x) {
                true => (right.x, left.x),
                false => (left.x, right.x)
            };

            HorizontalLine {
                left: Point::new(left_x, left.y),
                right: Point::new(right_x, left.y)
            }
        }
    }

    impl Line for VerticalLine {
        fn get_points(&self) -> Vec<Point> {
            let mut points = Vec::with_capacity(2);
            
            points.push(self.top);
            points.push(self.bottom);

            return points;
        }
    }

    impl Line for HorizontalLine {
        fn get_points(&self) -> Vec<Point> {
            let mut points = Vec::with_capacity(2);
            
            points.push(self.left);
            points.push(self.right);

            return points;
        }
    }

    impl fmt::Display for VerticalLine {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "|X={}, Y in [{}, {}]|", self.top.x, self.bottom.y, self.top.y)
        }
    }

    impl fmt::Display for HorizontalLine {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "|Y={}, X in [{}, {}]|", self.left.y, self.left.x, self.right.x)
        }
    }
}
