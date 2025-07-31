use std::fmt;
use crate::matrix::Matrix;

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
