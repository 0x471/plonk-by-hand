mod field;
mod elliptic_curve;

use elliptic_curve::{Point, PointExt};
use field::{Field101, Field101Ext};

fn main() {
    let g1 = Point::new(Field101::new(1), Field101::new(2)).unwrap();
    let g2 = PointExt::new(
        Field101Ext::new(Field101::new(1), Field101::new(0)),
        Field101Ext::new(Field101::new(2), Field101::new(0)),
    )
    .unwrap();

    println!("Subgroup 1 Generator (G1): {:?}", g1);
    println!("Subgroup 2 Generator (G2): {:?}", g2);
}
