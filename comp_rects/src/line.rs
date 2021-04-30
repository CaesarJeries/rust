

pub mod line {

    use crate::point::Point;
    use std::fmt;
    use std::cmp::PartialOrd;

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
            HorizontalLine {
                left,
                right
            }
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
