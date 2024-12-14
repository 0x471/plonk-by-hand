use crate::field::Field101;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    Infinity,
    Finite { x: Field101, y: Field101 },
}

impl Point {
    pub const A: Field101 = Field101::new(0);
    pub const B: Field101 = Field101::new(3);

    pub fn new(x: Field101, y: Field101) -> Self {
        if y.mul(y) != x.mul(x).mul(x).add(Self::B) {
            panic!("Point ({:?}, {:?}) is not on the curve!", x, y);
        }
        Point::Finite { x, y }
    }

    pub fn add(self, other: Point) -> Point {
        match (self, other) {
            (Point::Infinity, p) | (p, Point::Infinity) => p,
            (Point::Finite { x: x1, y: y1 }, Point::Finite { x: x2, y: y2 }) => {
                if x1 == x2 && y1 == y2 {
                    self.double()
                } else if x1 == x2 && y1 != y2 {
                    Point::Infinity
                } else {
                    let numerator = y2.sub(y1);
                    let denominator = x2.sub(x1);
                    let slope = numerator.div(denominator);
                    let x3 = slope.mul(slope).sub(x1.add(x2));
                    let y3 = slope.mul(x1.sub(x3)).sub(y1);
                    Point::Finite { x: x3, y: y3 }
                }
            }
        }
    }

    pub fn double(self) -> Point {
        match self {
            Point::Infinity => Point::Infinity,
            Point::Finite { x, y } => {
                let numerator = Field101::new(3).mul(x.mul(x));
                let denominator = Field101::new(2).mul(y);
                let slope = numerator.div(denominator);
                let x3 = slope.mul(slope).sub(x.add(x));
                let y3 = slope.mul(x.sub(x3)).sub(y);
                Point::Finite { x: x3, y: y3 }
            }
        }
    }

    pub fn invert(self) -> Point {
        match self {
            Point::Infinity => Point::Infinity,
            Point::Finite { x, y } => Point::Finite { x, y: Field101::new(0).sub(y) },
        }
    }
}
