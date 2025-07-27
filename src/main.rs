mod quantum_simulator;
mod matrix;

use quantum_simulator::{QuantumStateVector, cnot};
use matrix::Matrix;

fn main() {
    // let x = 1.0_f64 / (2.0_f64).sqrt();
    let psi = QuantumStateVector::new(&[
        0.619, // |00⟩
        0.309, // |01⟩
        0.722, // |10⟩
        0.008, // |11⟩
    ]);
    let psi = cnot(psi, 0, 1);

    println!("{psi}");

    let m = Matrix::new(
        vec!(
            vec![0.0, 1.0],
            vec![1.0, 0.0],
        )
    );


    println!("{}", m);




}
