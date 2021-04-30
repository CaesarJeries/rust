mod point;
mod line;
mod rectangle;

use crate::point::Point;
use crate::line::line::HorizontalLine;
use crate::line::line::VerticalLine;
use crate::rectangle::Rect;

fn main() {
    let p = Point::origin();

    println!("Hello, world!");
    println!("p = {}", p);

    let p1 = Point::new(12., 0.);
    println!("p.distance(p1) = {}", p.distance(&p1));

    let vline = VerticalLine::new(
        Point::new(2., 0.),
        Point::new(2., 42.)
    );

    let hline = HorizontalLine::new(
        Point::new(0., 23.14),
        Point::new(3.14, 23.14)
    );

    println!("vline: {}", vline);
    println!("hline: {}", hline);

    let r = Rect::new(Point::new(13., 13.), Point::new(42., 42.));
    println!("Rect: {}", r);

    let ip = vline.get_intersection(&hline);
    println!("intersection: {}", ip);

    let frame = Rect::new(Point::new(0., 0.), Point::new(100., 100.));
    let points = r.get_intersection_with_frame(&frame);
    
    println!("intersection points");
    for p in points {
        println!("{}", p);
    }
}
