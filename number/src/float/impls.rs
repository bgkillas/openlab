use crate::assign_each;
use crate::assign_impl;
use crate::float::complex::Complex;
use crate::float::real::Real;
use crate::traits::complex::ComplexImpl as _;
use crate::traits::real::RealImpl;
use core::ops::{Add, Div, Mul, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
impl<T: RealImpl> Add<Real<T>> for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Real<T>) -> Self::Output {
        Self::new(self.re + rhs, self.im)
    }
}
impl<T: RealImpl> Sub<Real<T>> for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Real<T>) -> Self::Output {
        Self::new(self.re - rhs, self.im)
    }
}
impl<T: RealImpl> Mul<Real<T>> for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Real<T>) -> Self::Output {
        Self::new(self.re * rhs, self.im * rhs)
    }
}
impl<T: RealImpl> Div<Real<T>> for Complex<T> {
    type Output = Self;
    fn div(self, rhs: Real<T>) -> Self::Output {
        Self::new(self.re / rhs, self.im / rhs)
    }
}
impl<T: RealImpl> Add<Complex<T>> for Real<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex::new(self + rhs.re, rhs.im)
    }
}
impl<T: RealImpl> Sub<Complex<T>> for Real<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Self::Output {
        Complex::new(self - rhs.re, -rhs.im)
    }
}
impl<T: RealImpl> Mul<Complex<T>> for Real<T> {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        Complex::new(self * rhs.re, self * rhs.im)
    }
}
impl<T: RealImpl> Div<Complex<T>> for Real<T> {
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Self::Output {
        let ab = self * rhs.re;
        let ac = self * rhs.im;
        Complex::new(ab, -ac) / rhs.abs_squared()
    }
}
assign_each!(Complex<T>, Real<T>, T: RealImpl);
