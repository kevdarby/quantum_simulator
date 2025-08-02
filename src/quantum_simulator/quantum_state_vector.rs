use crate::matrix::Matrix;
use std::fmt;

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
        let qsv = QuantumStateVector {
            state_vector: (Vec::from(vals)),
        };

        if !qsv.check_valid() {
            panic!("Invalid Quantum State Vector! The sum of states^2 is not 1.")
        }

        qsv
    }

    pub fn from_vec(v: Vec<f64>) -> Self {
        let qsv = QuantumStateVector { state_vector: v };

        if !qsv.check_valid() {
            panic!("Invalid Quantum State Vector! The sum of states^2 is not 1.")
        }

        qsv
    }

    pub fn to_matrix(&self) -> Matrix {
        Matrix::new(vec![self.state_vector.clone()])
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

    pub(crate) fn len(&self) -> usize {
        self.state_vector.len()
    }

    pub(crate) fn swap(&mut self, i: usize, j: usize) {
        self.state_vector.swap(i, j)
    }

    /// Measures the target bit, collapsing it to either 0 or 1.
    /// The probabilities for the other states are then normalized.
    ///
    /// # Arguments
    /// * 'target'  
    ///
    /// # Returns
    /// A tuple containing:
    /// - The measurement result (0 or 1).
    /// - The updated quantum state that is normalized without the qubit being measured.
    pub fn measure(mut self, target: usize) -> (u8, Self) {
        // make test case for making bell state, measure bit 0 = x, and asserting 1 is always x
        if target.pow(2) >= self.len() {
            panic!("Target index out of bounds: {}", target);
        }
        let mut zero_prob = 0.0;
        for i in 0..self.len() {
            let bit = (i >> target) & 1;
            if bit == 0 {
                zero_prob += self.state_vector[i].powi(2);
            }
        }
        let return_value: u8 = if rand::random::<f64>() < zero_prob {
            0
        } else {
            1
        };

        let mut total: f64 = 0.0;
        for i in 0..self.len() {
            let bit: u8 = ((i >> target) & 1) as u8;
            if bit != return_value {
                self.state_vector[i] = 0.0;
            }
            total += self.state_vector[i].powf(2.0);
        }
        total = total.sqrt();
        // Normalize the new state vector
        for i in 0..self.state_vector.len() {
            self.state_vector[i] /= total;
        }
        (return_value, self)
    }
}

impl fmt::Display for QuantumStateVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bit_len = (self.state_vector.len() as f64).log2().ceil() as usize;
        write!(f, "ψ = ")?;

        for i in 0..self.state_vector.len() {
            write!(
                f,
                "({:.5})|{:0width$b}>   ",
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

impl fmt::Debug for QuantumStateVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum_simulator::gates::{h, cnot};

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
    fn test_measure_00() {
        let psi = QuantumStateVector::new(&[1.0, 0.0, 0.0, 0.0]); // |00⟩
        let (bit, psi) = psi.measure(0);
        assert_eq!(bit, 0);
        assert_eq!(psi, QuantumStateVector::new(&[1.0, 0.0, 0.0, 0.0])); // |00⟩   
    }

    #[test]
    fn test_measure_01() {
        let psi = QuantumStateVector::new(&[0.0, 1.0, 0.0, 0.0]); // |01⟩
        let (bit, psi) = psi.measure(0);
        assert_eq!(bit, 1);
        assert_eq!(psi, QuantumStateVector::new(&[0.0, 1.0, 0.0, 0.0])); // |01⟩
    }

    #[test]
    fn test_measure_bell() {
        for _ in 0..10 {
            let psi = QuantumStateVector::new(&[1.0, 0.0, 0.0, 0.0]); // |00⟩
            let psi = h(psi, 1);
            let psi = cnot(psi, 1, 0);
            let (bit1, psi) = psi.measure(1);

            let (bit2, _) = psi.measure(0);
            assert_eq!(bit1, bit2)
        }
    }
}
