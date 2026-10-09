use crate::traits::complex::ComplexImpl;
pub struct ComplexUnits<T: ComplexImpl> {
    pub number: T,
    pub units: Units,
}
pub struct Units {
    pub units: [f64; 4],
}
