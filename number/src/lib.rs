#![feature(integer_casts)]
#![feature(cast_maybe_uninit)]
#![feature(complex_numbers)]
extern crate core;
pub mod complex;
pub mod float;
pub mod matrix;
#[cfg(test)]
mod matrix_test;
pub mod number;
pub mod real;
