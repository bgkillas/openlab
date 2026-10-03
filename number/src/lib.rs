#![feature(integer_casts)]
#![feature(cast_maybe_uninit)]
extern crate core;
mod assign_macros;
pub mod base;
pub mod complex;
pub mod float;
pub mod matrix;
#[cfg(test)]
mod matrix_test;
pub mod number;
pub mod real;
