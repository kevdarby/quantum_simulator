
mod quantum_state_vector;
fn main() {
    let x = 1.0_f64/(2.0_f64).sqrt();
    let _psi = quantum_state_vector::QuantumStateVector::new(&[x, x]);



}
