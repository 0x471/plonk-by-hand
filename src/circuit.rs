pub struct PlonkGate {
    pub ql: i32, // Selector for left input
    pub qr: i32, // Selector for right input
    pub qo: i32, // Selector for output
    pub qm: i32, // Selector for multiplication
    pub qc: i32, // Constant selector
    pub a: i32,  // Left wire input
    pub b: i32,  // Right wire input
    pub c: i32,  // Output wire
}

impl PlonkGate {
    pub fn evaluate(&self) -> bool {
        self.ql * self.a
            + self.qr * self.b
            + self.qo * self.c
            + self.qm * self.a * self.b
            + self.qc
            == 0
    }
}

pub struct PlonkCircuit {
    pub gates: Vec<PlonkGate>,
}

impl PlonkCircuit {
    pub fn new() -> Self {
        PlonkCircuit { gates: Vec::new() }
    }

    pub fn add_gate(&mut self, gate: PlonkGate) {
        self.gates.push(gate);
    }

    pub fn verify(&self) -> bool {
        self.gates.iter().all(|gate| gate.evaluate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pythagorean_circuit() {
        let mut circuit = PlonkCircuit::new();

        // Gate 1: a1 * b1 = c1
        circuit.add_gate(PlonkGate {
            ql: 0,
            qr: 0,
            qo: -1,
            qm: 1,
            qc: 0,
            a: 3,
            b: 3,
            c: 9,
        });

        // Gate 2: a2 * b2 = c2
        circuit.add_gate(PlonkGate {
            ql: 0,
            qr: 0,
            qo: -1,
            qm: 1,
            qc: 0,
            a: 4,
            b: 4,
            c: 16,
        });

        // Gate 3: a3 * b3 = c3
        circuit.add_gate(PlonkGate {
            ql: 0,
            qr: 0,
            qo: -1,
            qm: 1,
            qc: 0,
            a: 5,
            b: 5,
            c: 25,
        });

        // Gate 4: a4 + b4 = c4
        circuit.add_gate(PlonkGate {
            ql: 1,
            qr: 1,
            qo: -1,
            qm: 0,
            qc: 0,
            a: 9,
            b: 16,
            c: 25,
        });

        assert!(circuit.verify());
    }
}
