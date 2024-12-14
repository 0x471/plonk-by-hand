#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Field101 {
    value: u8,
}

impl Field101 {
    const MODULUS: u8 = 101;

    fn new(value: i32) -> Self {
        let mut v = value % Self::MODULUS as i32;
        if v < 0 {
            v += Self::MODULUS as i32;
        }
        Self { value: v as u8 }
    }

    fn add(self, other: Self) -> Self {
        Self::new((self.value + other.value) as i32)
    }

    fn sub(self, other: Self) -> Self {
        Self::new((self.value as i32 - other.value as i32) as i32)
    }

    fn mul(self, other: Self) -> Self {
        Self::new((self.value as i32 * other.value as i32) as i32)
    }

    fn inv(self) -> Self {
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

    fn div(self, other: Self) -> Self {
        self.mul(other.inv())
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::Field101;

    #[test]
    fn test_addition() {
        let a = Field101::new(50);
        let b = Field101::new(75);
        assert_eq!(a.add(b), Field101::new(24));
    }

    #[test]
    fn test_subtraction() {
        let a = Field101::new(50);
        let b = Field101::new(75);
        assert_eq!(a.sub(b), Field101::new(76));
    }

    #[test]
    fn test_multiplication() {
        let a = Field101::new(50);
        let b = Field101::new(75);
        assert_eq!(a.mul(b), Field101::new(13));
    }

    #[test]
    fn test_division() {
        let a = Field101::new(50);
        let b = Field101::new(75);
        assert_eq!(a.div(b), Field101::new(68));
    }

    #[test]
    fn test_inverse() {
        let a = Field101::new(50);
        assert_eq!(a.inv().mul(a), Field101::new(1));
    }
}
