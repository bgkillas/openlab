use crate::complex::ComplexImpl;
use crate::real::RealImpl;
use std::num::Complex;
impl RealImpl for f64 {}
impl ComplexImpl for Complex<f64> {}
