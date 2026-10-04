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
    fn new_real_from<T>(re: T) -> Self
    where
        T: Into<Self::Real>,
    {
        Self::new(<T as Into<Self::Real>>::into(re), Self::Real::zero())
    }
    #[must_use]
    fn new_imag_from<T>(re: T) -> Self
    where
        T: Into<Self::Real>,
    {
        Self::new(Self::Real::zero(), <T as Into<Self::Real>>::into(re))
    }
    #[must_use]
    fn new_from<T, K>(re: T, im: K) -> Self
    where
        T: Into<Self::Real>,
        K: Into<Self::Real>,
    {
        Self::new(
            <T as Into<Self::Real>>::into(re),
            <K as Into<Self::Real>>::into(im),
        )
    }
    #[must_use]
    fn new_real(re: Self::Real) -> Self {
        Self::new(re, Self::Real::zero())
    }
    #[must_use]
    fn new_imag(im: Self::Real) -> Self {
        Self::new(Self::Real::zero(), im)
    }
    #[must_use]
    fn new(re: Self::Real, im: Self::Real) -> Self;
    #[must_use]
    fn conjugate(self) -> Self;
    #[must_use]
    fn abs(self) -> Self::Real;
    #[must_use]
    fn abs_squared(self) -> Self::Real;
}
