use qip::prelude::*;
use std::{num::NonZeroUsize, result::Result};

fn main() -> Result<(), CircuitError> {
    let mut b = LocalBuilder::<f64>::default();
    let n = NonZeroUsize::new(3).unwrap();

    // Ancilla qubit
    let q = b.qubit();

    // Two qubits
    let ra = b.register(n);
    let rb = b.register(n);

    // Apply Hadamard gate to ancilla qubit
    let q = b.h(q);

    // Apply controlled swap gate to two qubits with ancilla qubit as control
    let mut cb = b.condition_with(q);
    let (ra, rb) = cb.swap(ra, rb)?;

    // Dissolve ancilla qubit (recover ownership)
    let q = cb.dissolve();

    // Apply Hadamard gate to ancilla qubit
    let q = b.h(q);

    // Measure ancilla qubit
    let (_, m_handle) = b.measure(q);

    // Prepare initial states for two qubits
    let states = [
        (0b000, 0b000),
        (0b000, 0b001),
        (0b001, 0b000),
        (0b001, 0b001),
    ];

    // Calculate final state
    let mut measured_states = vec![];
    for (ra_state, rb_state) in states.iter() {
        let (_, measured) = b.calculate_state_with_init([(&ra, *ra_state), (&rb, *rb_state)]);
        measured_states.push(measured.get_measurement(m_handle));
    }
    // let (_, measured) = b.calculate_state_with_init([(&ra, 0b000), (&rb, 0b001)]);

    // Print results
    for (result, p) in measured_states {
        println!("Measured: {} with probability: {:.4}", result, p);
    }

    Ok(())
}
