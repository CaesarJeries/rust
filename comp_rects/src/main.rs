
mod point;
mod line;

use point::Point;
use line::line::HorizontalLine;
use line::line::VerticalLine;

fn main() {
    let p = Point::new(0., 0.);

    println!("Hello, world!");
    println!("p = {}", p);

    let p1 = Point::new(12., 0.);
    println!("p.distance(p1) = {}", p.distance(&p1));

    let vline = VerticalLine::new(
        Point::new(13., 0.),
        Point::new(13., 42.)
    );

    let hline = HorizontalLine::new(
        Point::new(0., 23.14),
        Point::new(3.14, 23.14)
    );

    println!("vline: {}", vline);
    println!("hline: {}", hline);
}
