use crate::traits::base::BaseImpl;
use crate::traits::complex::ComplexImpl;
pub trait RealImpl: BaseImpl + From<u8> + From<f32> {
    type Complex: ComplexImpl;
    #[must_use]
    fn sqrt(self) -> Self;
    #[must_use]
    fn pow(self, rhs: Self) -> Self;
    #[must_use]
    fn root(self, rhs: Self) -> Self;
    #[must_use]
    fn cos(self) -> Self;
    #[must_use]
    fn sin(self) -> Self;
    #[must_use]
    fn sin_cos(self) -> (Self, Self);
    #[must_use]
    fn cosh(self) -> Self;
    #[must_use]
    fn sinh(self) -> Self;
    #[must_use]
    fn sinh_cosh(self) -> (Self, Self);
    #[must_use]
    fn exp(self) -> Self;
    #[must_use]
    fn ln(self) -> Self;
    #[must_use]
    fn atan2(self, x: Self) -> Self;
    #[must_use]
    fn hypot(self, y: Self) -> Self;
    fn is_positive(self) -> bool;
    #[must_use]
    fn abs(self) -> Self;
    #[must_use]
    fn fract(self) -> Self;
}
