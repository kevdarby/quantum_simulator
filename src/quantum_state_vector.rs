use std::fmt;

use crate::quantum_state_vector;
pub struct QuantumStateVector {
    state_vector: Vec<f64>,
}

impl QuantumStateVector {
    const EPSILON: f64 = 1e-3;
    /// Creates a new `QuantumStateVector` from a slice of `f64` values.
    ///
    /// # Arguments
    ///
    /// * `vals` - A slice of `f64` representing the quantum state amplitudes.
    ///
    /// # Panics
    ///
    /// This function will panic if the sum of the squares of the values is not 1,
    /// indicating an invalid quantum state vector.
    pub fn new(vals: &[f64]) -> Self {
        let x = QuantumStateVector {
            state_vector: (Vec::from(vals)),
        };

        if !x.check_valid() {
            panic!("Invalid Quantum State Vector! The sum of states^2 is not 1.")
        }

        x
    }

    /// Checks whether the quantum state vector is valid
    /// A quantum state vector is valid if
    /// 1. The 2norms sum to 1
    /// 2. The length is a power of 2
    fn check_valid(&self) -> bool {
        let mut norm = 0.0;

        for &val in &self.state_vector {
            norm += val.powi(2);
        }

        let length = self.state_vector.len();
        let power_of_two = length & (length - 1) == 0;

        let normilized = (norm - 1.0).abs() < Self::EPSILON;

        if !normilized {
            println!("norm is {norm}");
        }

        normilized && power_of_two
    }

    /// Applies a CNOT gate with the specified control and target qubits.
    ///
    /// # Arguments
    ///
    /// * `control` - The index of the control qubit starting from the right.
    /// * `target` - The index of the target qubit starting from the right.
    pub fn cnot(&mut self, control: usize, target: usize) {
        for i in 0..self.state_vector.len() {
            // Extract the bit value at position 'control' from index 'i':
            // For example, if control = 1 and i = 6 (binary 110), then:
            // (i >> control) & 1 = (110 >> 1) & 1 = (11) & 1 = 1
            // This tells us the state of the control qubit in the basis state represented by i.
            if i >> control & 1 == 1 {
                // if control bit is 1 then we flip the target bit
                let j = i ^ (1 << target); // flips the target bit 

                if i < j {
                    self.state_vector.swap(i, j);
                }
            }
        }
    }
}

impl fmt::Display for QuantumStateVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bit_len = (self.state_vector.len() as f64).log2().ceil() as usize;

        for i in 0..self.state_vector.len() {
            write!(
                f,
                "[{:.5} |{:0width$b}>]\n",
                self.state_vector[i],
                i,
                width = bit_len
            )?;
        }
        Ok(())
    }
}

impl PartialEq for QuantumStateVector {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.state_vector.len() {
            if (self.state_vector[i] - other.state_vector[i]).abs() >= Self::EPSILON {
                return false;
            }
        }
        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
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

    #[test]
    fn test_cnot_control0_target1_distinct() {
        let mut psi = quantum_state_vector::QuantumStateVector::new(&[
            0.619, // |00⟩
            0.309, // |01⟩
            0.722, // |10⟩
            0.008, // |11⟩
        ]);

        psi.cnot(0, 1);

        assert_eq!(
            psi.state_vector,
            vec![
                0.619, // |00⟩
                0.008, // |11⟩
                0.722, // |10⟩
                0.309, // |01⟩
            ]
        );
    }
}
