
#[cfg(test)]
mod line_tests {
    use comp_rects::point::Point;
    use comp_rects::line::line::HorizontalLine;
    use comp_rects::line::line::VerticalLine;

    #[test]
    #[should_panic(expected = "Line is not horizontal")]
    fn test_not_horizontal() {
        let _line = HorizontalLine::new(Point::new(1., 2.), Point::new(3., 4.));
    }
    
    #[test]
    #[should_panic(expected = "Line is not vertical")]
    fn test_not_vertical() {
        let _line = VerticalLine::new(Point::new(1., 2.), Point::new(3., 4.));
    }

    #[test]
    fn test_simple_case() {
        let _line = HorizontalLine::new(Point::new(0., 42.), Point::new(13., 42.));
        let _line = VerticalLine::new(Point::new(0., 0.), Point::new(0., 42.));
    }
}
