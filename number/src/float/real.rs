use crate::assign_each;
use crate::assign_impl;
use crate::base::BaseImpl;
use crate::float::complex::Complex;
use crate::real::RealImpl;
use core::ops::{Add, Div, Mul, Neg, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use core::ops::{Deref, DerefMut};
use replace_with::replace_with_or_abort;
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[repr(transparent)]
pub struct Real<T: RealImpl = f64>(pub T);
impl BaseImpl for f64 {}
impl RealImpl for f64 {
    type Complex = Complex<Self>;
    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
}
impl<T: RealImpl> BaseImpl for Real<T> {}
impl<T: RealImpl> RealImpl for Real<T> {
    type Complex = Complex<T>;
    fn sqrt(self) -> Self {
        Self(<T as RealImpl>::sqrt(*self))
    }
}
impl<T: RealImpl> Deref for Real<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: RealImpl> DerefMut for Real<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: RealImpl> Add for Real<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(*self + *rhs)
    }
}
impl<T: RealImpl> Sub for Real<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(*self - *rhs)
    }
}
impl<T: RealImpl> Mul for Real<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(*self * *rhs)
    }
}
impl<T: RealImpl> Div for Real<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(*self / *rhs)
    }
}
impl<T: RealImpl> Neg for Real<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-*self)
    }
}
assign_each!(Real<T>, Real<T>, T: RealImpl);
