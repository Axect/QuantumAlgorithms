use qip::{prelude::*, rand::distributions::Bernoulli};
use peroxide::fuga::*;
use std::result::Result;

fn main() {

}

fn gen_f_constant(n: usize) -> Box<dyn Fn(&[usize]) -> usize> {
    Box::new(move |x: &[usize]| -> usize {
        0
    })
}

fn gen_f_balanced(n: usize) -> Box<dyn Fn(&[usize]) -> usize> {
    let bernoulli = Bernoulli(0.5);
    let y = bernoulli.sample(2usize.pow((n-1) as u32)).iter().map(|&x| x as usize).collect::<Vec<usize>>();
    let y_not = y.iter().map(|&x| 1 - x).collect::<Vec<usize>>();
    let y = y.into_iter().chain(y_not.into_iter()).collect::<Vec<usize>>();

    Box::new(move |x: &[usize]| -> usize {
        let mut number = 0usize;
        for i in 0..n {
            number += x[i] * 2usize.pow((n-i-1) as u32);
        }
        y[number]
    })
}

fn real_mat_to_complex_mat<const N: usize>(mat: [f64; N]) -> [Complex<f64>; N] {
    let mut complex_mat = [Complex::new(0.0, 0.0); N];
    for (i, r) in mat.iter().enumerate() {
        complex_mat[i] = Complex::new(*r, 0.0);
    }
    complex_mat
}

#[allow(non_snake_case)]
fn U<F: Fn(usize) -> usize, const N: usize>(f: F, n: usize) -> [Complex<f64>; N] {
    let mut m = eye(2usize.pow(n as u32 + 1));

    todo!()
}