use qip::prelude::*;
use std::f64::consts::PI;
use std::result::Result;

const THETA: f64 = PI / 3.0;

fn main() -> Result<(), CircuitError> {
    let mut l = LocalBuilder::<f64>::default();
    let psi = l.qubit();
    let a = l.qubit();
    let b = l.qubit();

    // Prepare psi
    let psi = l.ry(psi, THETA);

    // Bell circuit on a, b
    let a = l.h(a);
    let (a, b) = l.cnot(a, b)?;

    // Reverse Bell circuit on psi, a
    let (psi, a) = l.cnot(psi, a)?;
    let psi = l.h(psi);

    // Measure psi and a
    let (_, handle_psi) = l.measure(psi);
    let (_, handle_a) = l.measure(a);

    // Alice get measurement result
    let (_, measured) = l.calculate_state();
    let (result_psi, _) = measured.get_measurement(handle_psi);
    let (result_a, _) = measured.get_measurement(handle_a);

    // Alice send result to Bob classically
    let b = if result_a == 1 { l.x(b) } else { b };
    let b = if result_psi == 1 { l.z(b) } else { b };

    // Bob measures b
    let (_, handle_b) = l.measure_stochastic(b);
    let (_, measured) = l.calculate_state();
    let result_b = measured.get_stochastic_measurement(handle_b);

    let ht = THETA / 2.0;
    let (s, c) = ht.sin_cos();
    println!(
        "
        Input: {:.4}|0> + {:.4}|1>

        Input probability vector:
            {:.4}, {:.4}

        Output probability vector:
            {:.4}, {:.4}
        ",
        c,
        s,
        c.powi(2),
        s.powi(2),
        result_b[0],
        result_b[1]
    );

    Ok(())
}
