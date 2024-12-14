#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field101 {
    pub value: u8,
}

impl Field101 {
    pub const MODULUS: u8 = 101;

    pub const fn new(value: i32) -> Self {
        let v = value.rem_euclid(Self::MODULUS as i32) as u8;
        Self { value: v }
    }

    pub fn add(self, other: Self) -> Self {
        Self::new((self.value + other.value) as i32)
    }

    pub fn sub(self, other: Self) -> Self {
        Self::new((self.value as i32 - other.value as i32) as i32)
    }

    pub fn mul(self, other: Self) -> Self {
        Self::new((self.value as i32 * other.value as i32) as i32)
    }

    pub fn inv(self) -> Self {
        let mut t = 0;
        let mut new_t = 1;
        let mut r = Self::MODULUS as i32;
        let mut new_r = self.value as i32;

        while new_r != 0 {
            let quotient = r / new_r;
            t = t - quotient * new_t;
            std::mem::swap(&mut t, &mut new_t);
            r = r - quotient * new_r;
            std::mem::swap(&mut r, &mut new_r);
        }

        if r > 1 {
            panic!("Element has no inverse!");
        }
        if t < 0 {
            t += Self::MODULUS as i32;
        }

        Self::new(t)
    }

    pub fn div(self, other: Self) -> Self {
        self.mul(other.inv())
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

// Extension field F101^2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field101Ext {
    pub a: Field101, // Real part
    pub b: Field101, // Coefficient of u
}

impl Field101Ext {
    pub const fn new(a: Field101, b: Field101) -> Self {
        Self { a, b }
    }

    pub fn add(self, other: Self) -> Self {
        Self {
            a: self.a.add(other.a),
            b: self.b.add(other.b),
        }
    }

    pub fn sub(self, other: Self) -> Self {
        Self {
            a: self.a.sub(other.a),
            b: self.b.sub(other.b),
        }
    }

    pub fn mul(self, other: Self) -> Self {
        let a = self.a.mul(other.a).sub(self.b.mul(other.b).mul(Field101::new(2))); // u^2 = -2
        let b = self.a.mul(other.b).add(self.b.mul(other.a));
        Self { a, b }
    }
    
    pub fn inv(self) -> Self {
        let denom = self.a.mul(self.a).add(self.b.mul(self.b).mul(Field101::new(2))); // a^2 + 2b^2
        if denom == Field101::new(0) {
            panic!("Element has no inverse!");
        }
        let denom_inv = denom.inv();
        let a = self.a.mul(denom_inv);
        let b = self.b.mul(denom_inv).mul(Field101::new(0).sub(Field101::new(1))); // -b
        Self { a, b }
    }
    

    pub fn div(self, other: Self) -> Self {
        self.mul(other.inv())
    }
}
