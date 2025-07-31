mod matrix;
mod quantum_simulator;

use quantum_simulator::{QuantumStateVector, cnot, h};
// use matrix::Matrix;

fn main() {
    // let x = 1.0_f64 / (2.0_f64).sqrt();
    // let psi = QuantumStateVector::new(&[
    //     0.619, // |00⟩
    //     0.309, // |01⟩
    //     0.722, // |10⟩
    //     0.008, // |11⟩
    // ]);
    // let psi = QuantumStateVector::new(&[
    //     1.0, // |00⟩
    //     0.0, // |01⟩
    //     0.0, // |10⟩
    //     0.0, // |11⟩
    // ]);
    let psi = QuantumStateVector::new(&[1.0, 0.0, 0.0, 0.0]); // |00⟩
    let psi = h(psi, 1); // Apply Hadamard to the leftmost qubit
    println!("psi state: {}", psi);
    let psi = cnot(psi, 1, 0); // Apply CNOT with control on the leftmost qubit and target on the rightmost qubit

    println!("Final state: {}", psi);
}
