use crate::{matrix::Matrix, quantum_simulator::QuantumStateVector};
use crate::matrix::matrix_operations::*;

/// Applies a CNOT gate with the specified control and target qubits.
/// CNOT gate swaps the target bit if the control bit is 1.
/// The qubit indicies are little endian.
///
/// # Arguments
///
/// * `control` - The index of the control qubit (little endian)
/// * `target` - The index of the target qubit (little endian)
pub fn cnot(mut psi: QuantumStateVector, control: usize, target: usize) -> QuantumStateVector {
    for i in 0..psi.len() {
        // Extract the bit value at position 'control' from index 'i':
        // For example, if control = 1 and i = 6 (binary 110), then:
        // (i >> control) & 1 = (110 >> 1) & 1 = (11) & 1 = 1
        // This tells us the state of the control qubit in the basis state represented by i.
        if i >> control & 1 == 1 {
            // if control bit is 1 then we flip the target bit
            let j = i ^ (1 << target); // flips the target bit 

            if i < j {
                psi.swap(i, j);
            }
        }
    }

    psi
}

/// Applies a Hadamard gate on the target qubit of psi.
/// 
/// # Arguments
///
/// * `target` - The index of the target qubit (little endian)
pub fn h(psi: QuantumStateVector, target: usize) -> QuantumStateVector {
    let num_qubits = (psi.len() as f64).log2() as usize;

    let mut op = Matrix::new(vec![vec![1.0]]); 

    for i in 0..num_qubits {
        if i == target {
            op = tensor_product(op, Matrix::hadamard());
        } else {
            op = tensor_product(op, Matrix::identity(2));
        }
    }

    let final_matrix = dot_product(op, transpose(psi.to_matrix())).m.remove(0);
    println!("Final matrix: {:?}", final_matrix);
    QuantumStateVector::from_vec(final_matrix)
}


#[cfg(test)]
mod tests {
    use super::super::QuantumStateVector;
    use super::*;

    #[test]
    fn test_cnot_control0_target1_2qubit() {
        let mut psi = QuantumStateVector::new(&[
            0.619, // |00⟩
            0.309, // |01⟩
            0.722, // |10⟩
            0.008, // |11⟩
        ]);

        psi = cnot(psi, 0, 1); // swaps 01 and 11 values
        assert_eq!(
            psi,
            QuantumStateVector::new(&[
                0.619, // |00⟩
                0.008, // |11⟩
                0.722, // |10⟩
                0.309, // |01⟩
            ])
        );
    }

    #[test]
    fn test_cnot_control0_target2_3qubit() {
        let mut psi = QuantumStateVector::new(&[
            0.070, // |000⟩
            0.140, // |001⟩
            0.210, // |010⟩
            0.280, // |011⟩
            0.350, // |100⟩
            0.420, // |101⟩
            0.490, // |110⟩
            0.560, // |111⟩
        ]);

        psi = cnot(psi, 0, 2); // control qubit 0, target qubit 2

        assert_eq!(
            psi,
            QuantumStateVector::new(&[
                0.070, // |000⟩
                0.420, // |001⟩ swapped with |101⟩
                0.210, // |010⟩
                0.560, // |011⟩ swapped with |111⟩
                0.350, // |100⟩
                0.140, // |101⟩ swapped with |001⟩
                0.490, // |110⟩
                0.280, // |111⟩ swapped with |011⟩
            ])
        );
    }
    #[test]
    fn test_hadamard_on_1qubit_zero() {
        let psi = QuantumStateVector::new(&[1.0, 0.0]);
        let result = h(psi, 0);
        let expected = QuantumStateVector::new(&[1.0 / 2f64.sqrt(), 1.0 / 2f64.sqrt()]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hadamard_on_2qubit_target1() {
        let psi = QuantumStateVector::new(&[1.0, 0.0, 0.0, 0.0]); // |00⟩
        let result = h(psi, 1);
        let expected = QuantumStateVector::new(&[
            1.0 / 2f64.sqrt(), // |00⟩
            0.0,               // |01⟩
            1.0 / 2f64.sqrt(), // |10⟩
            0.0,               // |11⟩
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hadamard_on_2qubit_target0() {
        let psi = QuantumStateVector::new(&[1.0, 0.0, 0.0, 0.0]); // |00⟩
        let result = h(psi, 0);
        let expected = QuantumStateVector::new(&[
            1.0 / 2f64.sqrt(), // |00⟩
            1.0 / 2f64.sqrt(), // |01⟩
            0.0,               // |10⟩
            0.0,               // |11⟩
        ]);
        assert_eq!(result, expected);
    }

    // #[test]
    // fn test_hadamard_zero() {
    //     let mut psi = QuantumStateVector::new(&[1.0, 0.0]);

    //     // psi = h(psi);
    // }
}
