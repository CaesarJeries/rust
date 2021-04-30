

pub mod line {

    use crate::point::Point;
    use std::fmt;

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
            VerticalLine {
                top,
                bottom
            }
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
