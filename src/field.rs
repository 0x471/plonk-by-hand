#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field101 {
    value: u8,
}

impl Field101 {
    const MODULUS: u8 = 101;

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
}
