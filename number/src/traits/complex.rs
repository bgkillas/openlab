use crate::traits::base::BaseImpl;
use crate::traits::real::RealImpl;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
pub trait ComplexImpl:
    BaseImpl
    + Copy
    + Add<Self::Real, Output = Self>
    + AddAssign<Self::Real>
    + Sub<Self::Real, Output = Self>
    + SubAssign<Self::Real>
    + Mul<Self::Real, Output = Self>
    + MulAssign<Self::Real>
    + Div<Self::Real, Output = Self>
    + DivAssign<Self::Real>
where
    Self::Real: Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>,
{
    type Real: RealImpl;
    #[must_use]
    fn new(re: Self::Real, im: Self::Real) -> Self;
    #[must_use]
    fn conjugate(self) -> Self;
    #[must_use]
    fn abs(self) -> Self::Real;
    #[must_use]
    fn abs_squared(self) -> Self::Real;
}
