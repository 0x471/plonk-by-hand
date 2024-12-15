pub struct PythagoreanCircuit {
    pub x1: u32,
    pub x2: u32,
    pub x3: u32,
    pub x4: u32,
    pub x5: u32,
    pub x6: u32,
}

impl PythagoreanCircuit {
    pub fn new(x1: u32, x3: u32, x5: u32) -> Self {
        let x2 = x1 * x1; // x1^2 = x2
        let x4 = x3 * x3; // x3^2 = x4
        let x6 = x5 * x5; // x5^2 = x6

        // Ensure x2 + x4 = x6
        assert_eq!(x2 + x4, x6, "Pythagorean triple condition failed!");

        PythagoreanCircuit { x1, x2, x3, x4, x5, x6 }
    }

    pub fn verify(&self) -> bool {
        self.x1 * self.x1 == self.x2 && // Gate 1: x1^2 = x2
        self.x3 * self.x3 == self.x4 && // Gate 2: x3^2 = x4
        self.x5 * self.x5 == self.x6 && // Gate 3: x5^2 = x6
        self.x2 + self.x4 == self.x6    // Gate 4: x2 + x4 = x6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pythagorean_circuit() {
        // (3, 4, 5)
        let circuit = PythagoreanCircuit::new(3, 4, 5);
        assert!(circuit.verify());

        // (5, 12, 13)
        let circuit = PythagoreanCircuit::new(5, 12, 13);
        assert!(circuit.verify());

        // Invalid example
        // let circuit = PythagoreanCircuit::new(3, 4, 6);
        // assert!(!circuit.verify());
    }
}
