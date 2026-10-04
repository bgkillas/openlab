use crate::assign_impl;
use crate::float::complex::Complex;
use crate::traits::assign::NegAssign;
use crate::traits::base::BaseImpl;
use crate::traits::real::RealImpl;
use crate::{assign_each, assign_neg};
use core::iter::{Product, Sum};
use core::ops::{Add, Div, Mul, Neg, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use core::ops::{Deref, DerefMut};
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[repr(transparent)]
pub struct Real<T: RealImpl = f64>(pub T);
impl BaseImpl for f64 {
    fn one() -> Self {
        1.0
    }
}
impl<T: Into<f64>> From<T> for Real<f64> {
    fn from(value: T) -> Self {
        Self(<T as Into<f64>>::into(value))
    }
}
assign_neg!(f64);
impl RealImpl for f64 {
    type Complex = Complex<Self>;
    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
}
impl<T: RealImpl> BaseImpl for Real<T> {
    fn one() -> Self {
        Self(T::one())
    }
}
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
impl<T: RealImpl> Sum for Real<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |total, v| total + v)
    }
}
impl<T: RealImpl> Product for Real<T> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::one(), |total, v| total * v)
    }
}
assign_each!(Real<T>, Self, T: RealImpl);
