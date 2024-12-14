mod field;
mod elliptic_curve;

use elliptic_curve::Point;
use field::Field101;

fn main() {
    let g = Point::new(Field101::new(1), Field101::new(2));

    let doubled = g.double();
    let inverted = g.invert();

    println!("G: {:?}", g);
    println!("2G (Doubling): {:?}", doubled);
    println!("-G (Inversion): {:?}", inverted);
}
