use peroxide::fuga::*;
use qip::prelude::*;
use quantum_algorithm::apply_if;
use seq_macro::seq;
use std::result::Result;

const N: usize = 20usize;

macro_rules! bernstein_vazirani {
    ($l:ident) => {{
        let a = $l.qubit();
        let a = $l.x(a);
        let a = $l.h(a);

        seq!(Q in 0..20usize {
            let q~Q = $l.qubit();
            let q~Q = $l.h(q~Q);
        });

        let b = Uniform(1i32, 2i32.pow(N as u32)-1).sample(1)[0] as usize;
        let b_str = format!("{:0width$b}", b, width = N);
        let b_chars = b_str.chars().collect::<Vec<char>>();

        // Oracle
        let q = {
            seq!(Q in 0..20 {
                #[allow(unused_variables)]
                let (q~Q, a) = apply_if!(b_chars[Q] == '1', $l.cnot(q~Q, a)?, (q~Q, a));
            });
            let mut q = vec![];
            seq!(Q in 0..20usize {
                q.push(q~Q);
            });
            $l.merge_registers(q).unwrap()
        };
        let q = $l.h(q);
        (q, b_str)
    }};
}

fn main() -> Result<(), CircuitError> {
    let mut l = LocalBuilder::<f64>::default();

    let (q, b_str) = bernstein_vazirani!(l);

    let (_, handle) = l.measure(q);

    let (_, measured) = l.calculate_state();
    let (result, p) = measured.get_measurement(handle);

    // Reversing result (little-endian)
    let result = format!("{:0width$b}", result, width = N)
        .chars()
        .rev()
        .collect::<String>();

    println!("Measured: {} with probability {:.4}", result, p,);
    println!("Expected: {}", b_str);

    Ok(())
}
