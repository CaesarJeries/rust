

#[cfg(test)]
mod point_tests {

    use comp_rects::point::Point;
   
    #[test]
    fn test_eq() {
        let p1 = Point::new(0., 0.);
        assert_eq!(p1, p1);

        let p2 = Point::new(0., 0.);
        assert_eq!(p1, p2);

        let p3 = Point::new(42., 42.);
        assert_ne!(p1, p3);
    }
}