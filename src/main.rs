mod quantum_state_vector;
mod gates;
fn main() {
    // let x = 1.0_f64 / (2.0_f64).sqrt();
    let mut psi = quantum_state_vector::QuantumStateVector::new(&[
        0.619, // |00⟩
        0.309, // |01⟩
        0.722, // |10⟩
        0.008, // |11⟩
    ]);
    println!("{psi}");
    psi.cnot(0, 1);
    println!("{psi}");
}
