


    impl super::quantum_state_vector::QuantumStateVector {
        
        /// Applies a CNOT gate with the specified control and target qubits.
        ///
        /// # Arguments
        ///
        /// * `control` - The index of the control qubit starting from the right.
        /// * `target` - The index of the target qubit starting from the right.
        pub fn cnot(&mut self, control: usize, target: usize) {
            for i in 0..self.len() {
                // Extract the bit value at position 'control' from index 'i':
                // For example, if control = 1 and i = 6 (binary 110), then:
                // (i >> control) & 1 = (110 >> 1) & 1 = (11) & 1 = 1
                // This tells us the state of the control qubit in the basis state represented by i.
                if i >> control & 1 == 1 {
                    // if control bit is 1 then we flip the target bit
                    let j = i ^ (1 << target); // flips the target bit 
    
                    if i < j {
                        self.swap(i, j);
                    }
                }
            }
        }
    }