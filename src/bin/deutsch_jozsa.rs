use qip::prelude::*;
use peroxide::fuga::*;
use std::result::Result;
use dialoguer::{theme::ColorfulTheme, Input, Select};

const N: usize = 10;

macro_rules! apply_if {
    ($cond:expr, $action:expr, $val:expr) => {
        if $cond {
            $action
        } else {
            $val
        }
    };
}

macro_rules! apply_oracle {
    ($l:ident;$n:ident;$case:expr;$($q:ident=>$i:expr),+; $a:ident) => {{
        $(let $q = $l.h($q);)+
        let $a = $l.h($a);

        // Oracle
        let (q, a) = match $case {
            Case::Balanced => {
                let b = Uniform(1i32, 2i32.pow($n as u32)-1).sample(1)[0] as usize;
                let b_str = format!("{:0width$b}", b, width = $n);
                let b_chars = b_str.chars().collect::<Vec<char>>();
                $(let $q = apply_if!(b_chars[$i] == '1', $l.x($q), $q);)+
                $(let ($q, $a) = $l.cnot($q, $a)?;)+
                $(let $q = apply_if!(b_chars[$i] == '1', $l.x($q), $q);)+

                let q = $l.merge_registers([$($q),+]).unwrap();
                let a = $a;
                (q, a)
            }
            Case::Constant => {
                let b = Uniform(0i32, 1i32).sample(1)[0] as usize;
                let $a = apply_if!(b == 1, $l.x($a), $a);
                let q = $l.merge_registers([$($q),+]).unwrap();
                let a = $a;
                (q, a)
            }
        };
        (q, a)
    }};
}

fn main() -> Result<(), CircuitError> {
    let case = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose case")
        .default(0)
        .items(&[Case::Balanced, Case::Constant])
        .interact()
        .unwrap();
    let case = match case {
        0 => Case::Balanced,
        _ => Case::Constant
    };
    
    let mut l = LocalBuilder::<f64>::default();
    let q1 = l.qubit();
    let q2 = l.qubit();
    let q3 = l.qubit();
    let q4 = l.qubit();
    let q5 = l.qubit();
    let q6 = l.qubit();
    let q7 = l.qubit();
    let q8 = l.qubit();
    let q9 = l.qubit();
    let q10 = l.qubit();
    let a = l.qubit();
    let a = l.x(a);

    let (q, a) = apply_oracle!(l; N; case; q1=>0, q2=>1, q3=>2, q4=>3, q5=>4, q6=>5, q7=>6, q8=>7, q9=>8, q10=>9; a);

    let q = l.h(q);

    let (_, handle) = l.measure(q);

    // Calculate final state
    let (_, measured) = l.calculate_state();
    let (result, p) = measured.get_measurement(handle);
    println!(
        "Measured: {:0width$b} with probability {:.4}",
        result,
        p,
        width = N
    );

    Ok(())
}

#[derive(Debug, Copy, Clone)]
pub enum Case {
    Balanced,
    Constant
}

impl std::fmt::Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Case::Balanced => write!(f, "Balanced"),
            Case::Constant => write!(f, "Constant"),
        }
    }
}
