use qip::prelude::*;
use std::result::Result;

fn main() -> Result<(), CircuitError> {
    let mut l = LocalBuilder::<f64>::default();

    let e1 = l.qubit();
    let e2 = l.qubit();
    let a = l.qubit();
    let b = l.qubit();

    // Hadamard gate on a
    let a = l.h(a);

    // CNOT gate on a and b
    let (a, b) = l.cnot(a, b)?;

    // CZ gate on e_1 and a
    let mut ce1 = l.condition_with(e1);
    let a = ce1.z(a);
    let e1 = ce1.dissolve();

    // CNOT gate on e_2 and a
    let (e2, a) = l.cnot(e2, a)?;

    // CNOT gate on a and b
    let (a, b) = l.cnot(a, b)?;

    // Hadamard gate on a
    let a = l.h(a);

    // Measure a and b
    let (_, m_handle_a) = l.measure(a);
    let (_, m_handle_b) = l.measure(b);

    // Calculate final state
    let alice_states = [(0, 0), (0, 1), (1, 0), (1, 1)];

    for (alice1, alice2) in alice_states.into_iter() {
        let (_, measured) = l.calculate_state_with_init([(&e1, alice1), (&e2, alice2)]);
        let (result_a, p_a) = measured.get_measurement(m_handle_a);
        let (result_b, p_b) = measured.get_measurement(m_handle_b);
        println!(
            "Input: {}{}, Measured: {}{} with probability: {:.4}",
            alice1,
            alice2,
            result_a,
            result_b,
            p_a * p_b
        );
    }

    Ok(())
}
