use dialoguer::{theme::ColorfulTheme, Select};
use qip::prelude::*;
use std::result::Result;

#[allow(unused_variables)]
fn main() -> Result<(), CircuitError> {
    // Select f(0) and f(1)
    let f0 = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose f(0)")
        .default(0)
        .items(&[0, 1])
        .interact()
        .unwrap();
    let f1 = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose f(1)")
        .default(0)
        .items(&[0, 1])
        .interact()
        .unwrap();

    // Make function f from inputs
    let f = |x: usize| -> usize {
        if x == 0 {
            f0
        } else {
            f1
        }
    };

    // Generate U_f from given f
    let u_f = U(f);

    // Make a circuit
    let mut l = LocalBuilder::<f64>::default();
    let q1 = l.qubit();
    let q2 = l.qubit();

    // Apply X gate to second qubit to make it |1>
    let q2 = l.x(q2);

    // Apply Hadamard gate to each qubit
    let q1 = l.h(q1);
    let q2 = l.h(q2);

    // Merge qubits into a register
    let q = l.merge_two_registers(q1, q2);

    // Apply U_f to the register
    let q = l.apply_matrix(q, u_f)?;

    // Split register into two qubits
    let (q1, q2) = match l.split_first_qubit(q) {
        (Some(q1), q2) => (q1, q2),
        (None, _) => panic!("Failed to split register into two qubits"),
    };

    // Apply Hadamard gate to first qubit
    let q1 = l.h(q1);

    // Measure first qubit
    let (_, m_handle) = l.measure(q1);
    let (_, m) = l.calculate_state();
    let (result, p) = m.get_measurement(m_handle);

    println!("Measured: {} with probability: {:.4}", result, p);

    // Deutsch's algorithm
    // if result in 0 state, f is constant
    // if result in 1 state, f is balanced
    println!(
        "
        f(0) = {}, f(1) = {}
        f is {}.
    ",
        f0,
        f1,
        if result == 0 { "constant" } else { "balanced" }
    );

    Ok(())
}

fn real_mat_to_complex_mat<const N: usize>(mat: [f64; N]) -> [Complex<f64>; N] {
    let mut complex_mat = [Complex::new(0.0, 0.0); N];
    for (i, r) in mat.iter().enumerate() {
        complex_mat[i] = Complex::new(*r, 0.0);
    }
    complex_mat
}

#[allow(non_snake_case)]
fn U<F: Fn(usize) -> usize>(f: F) -> [Complex<f64>; 16] {
    let f0 = f(0) as f64;
    let f1 = f(1) as f64;

    real_mat_to_complex_mat([
        1.0 - f0,
        f0,
        0.0,
        0.0,
        f0,
        1.0 - f0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0 - f1,
        f1,
        0.0,
        0.0,
        f1,
        1.0 - f1,
    ])
}
