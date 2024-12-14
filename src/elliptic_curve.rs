use crate::field::{Field101, Field101Ext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: Field101,
    pub y: Field101,
}

impl Point {
    pub const A: Field101 = Field101::new(0);
    pub const B: Field101 = Field101::new(3);

    pub fn new(x: Field101, y: Field101) -> Option<Self> {
        if y.mul(y) != x.mul(x).mul(x).add(Self::B) {
            None
        } else {
            Some(Point { x, y })
        }
    }

    pub fn add(self, other: Point) -> Option<Point> {
        if self.x == other.x && self.y != other.y {
            return None;
        }

        if self.x == other.x && self.y == other.y {
            return self.double();
        }

        let numerator = other.y.sub(self.y);
        let denominator = other.x.sub(self.x);
        let slope = numerator.div(denominator);
        let x3 = slope.mul(slope).sub(self.x.add(other.x));
        let y3 = slope.mul(self.x.sub(x3)).sub(self.y);
        Some(Point { x: x3, y: y3 })
    }

    pub fn double(self) -> Option<Point> {
        if self.y == Field101::new(0) {
            return None;
        }

        let numerator = Field101::new(3).mul(self.x.mul(self.x));
        let denominator = Field101::new(2).mul(self.y);
        let slope = numerator.div(denominator);
        let x3 = slope.mul(slope).sub(self.x.add(self.x));
        let y3 = slope.mul(self.x.sub(x3)).sub(self.y);
        Some(Point { x: x3, y: y3 })
    }

    pub fn invert(self) -> Point {
        Point {
            x: self.x,
            y: Field101::new(0).sub(self.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointExt {
    pub x: Field101Ext,
    pub y: Field101Ext,
}

impl PointExt {
    pub const A: Field101Ext = Field101Ext::new(Field101::new(0), Field101::new(0));
    pub const B: Field101Ext = Field101Ext::new(Field101::new(3), Field101::new(0));

    pub fn new(x: Field101Ext, y: Field101Ext) -> Option<Self> {
        let lhs = y.mul(y);
        let rhs = x.mul(x).mul(x).add(Self::B);
        if lhs != rhs {
            None
        } else {
            Some(PointExt { x, y })
        }
    }

    pub fn add(self, other: PointExt) -> Option<PointExt> {
        if self.x == other.x && self.y != other.y {
            return None;
        }

        if self.x == other.x && self.y == other.y {
            return self.double();
        }

        let numerator = other.y.sub(self.y);
        let denominator = other.x.sub(self.x);
        let slope = numerator.div(denominator);
        let x3 = slope.mul(slope).sub(self.x.add(other.x));
        let y3 = slope.mul(self.x.sub(x3)).sub(self.y);
        Some(PointExt { x: x3, y: y3 })
    }

    pub fn double(self) -> Option<PointExt> {
        if self.y == Field101Ext::new(Field101::new(0), Field101::new(0)) {
            return None;
        }

        let numerator = Field101Ext::new(Field101::new(3), Field101::new(0)).mul(self.x.mul(self.x));
        let denominator = Field101Ext::new(Field101::new(2), Field101::new(0)).mul(self.y);
        let slope = numerator.div(denominator);
        let x3 = slope.mul(slope).sub(self.x.add(self.x));
        let y3 = slope.mul(self.x.sub(x3)).sub(self.y);
        Some(PointExt { x: x3, y: y3 })
    }
}