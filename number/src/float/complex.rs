use crate::assign_each;
use crate::assign_impl;
use crate::assign_neg;
use crate::float::real::Real;
use crate::traits::assign::NegAssign;
use crate::traits::base::BaseImpl;
use crate::traits::complex::ComplexImpl;
use crate::traits::real::RealImpl;
use core::ops::{Add, Div, Mul, Neg, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Complex<T: RealImpl = f64> {
    pub re: Real<T>,
    pub im: Real<T>,
}
impl<T: RealImpl> BaseImpl for Complex<T> {}
impl<T: RealImpl> ComplexImpl for Complex<T> {
    type Real = Real<T>;
    fn new(re: Real<T>, im: Real<T>) -> Self {
        Self { re, im }
    }
    fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
    fn abs(self) -> Real<T> {
        self.abs_squared().sqrt()
    }
    fn abs_squared(self) -> Real<T> {
        self.re * self.re + self.im * self.im
    }
}
impl<T: RealImpl> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}
impl<T: RealImpl> Add<Self> for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}
impl<T: RealImpl> Sub<Self> for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}
impl<T: RealImpl> Mul<Self> for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let Self { re: a, im: b } = self;
        let Self { re: c, im: d } = rhs;
        let ac = a * c;
        let bd = b * d;
        let ad = a * d;
        let bc = b * c;
        Self::new(ac - bd, bc + ad)
    }
}
impl<T: RealImpl> Div<Self> for Complex<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let Self { re: a, im: b } = self;
        let Self { re: c, im: d } = rhs;
        let ac = a * c;
        let bd = b * d;
        let ad = a * d;
        let bc = b * c;
        Self::new(ac + bd, bc - ad) / rhs.abs_squared()
    }
}
assign_each!(Complex<T>, Self, T: RealImpl);
