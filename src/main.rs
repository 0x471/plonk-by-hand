mod field;
mod elliptic_curve;

use elliptic_curve::Point;
use field::Field101;

fn main() {
    let g = Point::new(Field101::new(1), Field101::new(2)).unwrap();
    println!("Subgroup Generated by G ({:?}):", g);

    let mut subgroup = Vec::new();
    let mut current = Some(g);

    for i in 0.. {
        if let Some(point) = current {
            if subgroup.contains(&point) {
                break;
            }

            println!("{}G = {:?}", i, point);
            subgroup.push(point);
            current = point.add(g);
        } else {
            println!("{}G = Identity", i);
            break;
        }
    }

    println!("\nSubgroup Order: {}", subgroup.len());
}
