#![feature(integer_casts)]
use core::array;
use core::hint::black_box;
use openlab::parser::number::float::complex::Complex;
use openlab::parser::number::matrix::dense::Matrix;
use openlab::parser::number::traits::matrix::MatrixImpl as _;
fn main() {
    let mut a: Matrix<Complex<f64>> =
        black_box(Matrix::from(array::from_fn::<[u16; 64], 24, _>(|i| {
            array::from_fn::<u16, 64, _>(|j| (i * 24 + j).strict_cast())
        })));
    let mut tmr = std::time::Instant::now();
    let mut m: Matrix<Complex<f64>> =
        black_box(Matrix::from(array::from_fn::<[u16; 64], 24, _>(|i| {
            array::from_fn::<u16, 64, _>(|j| (i * 24 + j).strict_cast())
        })));
    println!("{}", tmr.elapsed().as_nanos());
    tmr = std::time::Instant::now();
    m.transpose();
    println!("{}", tmr.elapsed().as_nanos());
    tmr = std::time::Instant::now();
    a.mul_assign(&m).unwrap();
    println!("{}", tmr.elapsed().as_nanos());
    black_box(m);
    black_box(a);
}
