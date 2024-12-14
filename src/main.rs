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

    // G2 in F101^2
    let x2 = Field101Ext::new(Field101::new(36), Field101::new(0));
    let y2 = Field101Ext::new(Field101::new(0), Field101::new(31));
    let g2 = PointExt::new(x2, y2).unwrap();
    
    println!("G2: ({:?} + {:?}u, {:?} + {:?}u)", 
        g2.x.a.value(), g2.x.b.value(),
        g2.y.a.value(), g2.y.b.value());

    // Double G2
    let double_g2 = g2.double().unwrap();
    println!("2G2: ({:?} + {:?}u, {:?} + {:?}u)", 
        double_g2.x.a.value(), double_g2.x.b.value(),
        double_g2.y.a.value(), double_g2.y.b.value());
    
}
