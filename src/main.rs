mod quantum_simulator;
mod matrix;

use quantum_simulator::{QuantumStateVector, cnot, h};
use matrix::Matrix;

fn main() {
    // let x = 1.0_f64 / (2.0_f64).sqrt();
    // let psi = QuantumStateVector::new(&[
    //     0.619, // |00⟩
    //     0.309, // |01⟩
    //     0.722, // |10⟩
    //     0.008, // |11⟩
    // ]);
    let psi = QuantumStateVector::new(&[
        1.0, // |00⟩
        0.0, // |01⟩
        0.0, // |10⟩
        0.0, // |11⟩
    ]);

    let m = h(psi, 0);


    println!("{}", m);




}
