mod field;
mod elliptic_curve;

use elliptic_curve::{Point, PointExt};
use field::{Field101, Field101Ext};

fn main() {
    // Subgroup 1 (G1) in F101
    let g1 = Point::new(Field101::new(1), Field101::new(2)).unwrap();
    println!("Subgroup 1 Generator (G1): {:?}", g1);

    let mut subgroup1 = Vec::new();
    let mut current_g1 = Some(g1);

    while let Some(point) = current_g1 {
        if subgroup1.contains(&point) {
            break;
        }
        subgroup1.push(point);
        current_g1 = point.add(g1);
    }
    println!("Subgroup 1 Elements:");
    for point in &subgroup1 {
        println!("({:?}, {:?})", point.x.value(), point.y.value());
    }
}
