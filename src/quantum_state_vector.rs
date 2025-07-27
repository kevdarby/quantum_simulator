pub struct QuantumStateVector {
    state_vector: Vec<f64>,
}

impl QuantumStateVector {
    pub fn new(vals: &[f64]) -> Self {
        let x = QuantumStateVector {
            state_vector: (Vec::from(vals)),
        };

        if !x.check_valid() {
            panic!("Invalid Quantum State Vector! The sum of states^2 is not 1.")
        }

        x
    }

    pub fn cnot(&self) {}

    fn check_valid(&self) -> bool {
        let epsilon = 1e-10;
        let norm = &self
            .state_vector
            .iter()
            .fold(0.0, |acc, val| acc + val.powi(2));

        (norm - 1.0).abs() < epsilon
    }

    // pub fn push(&mut self, val: f64) {
    //     self.state_vector.push(val);
    // }

    // pub fn get_state(&self, index: usize) -> f64 {
    //     self.state_vector[index]
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_qsv() {
        let x = 1.0_f64 / (2.0_f64).sqrt();
        let _psi = QuantumStateVector::new(&[x, x]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_qsv() {
        let x = 1.0_f64 / (2.0_f64).sqrt();
        let _psi = QuantumStateVector::new(&[x, x, x]);
    }
}
