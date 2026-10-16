use crate::assign_each;
use crate::assign_impl;
use crate::assign_neg;
use crate::float::real::Real;
use crate::traits::assign::NegAssign;
use crate::traits::base::BaseImpl;
use crate::traits::complex::ComplexImpl;
use crate::traits::number::NumberImpl as _;
use crate::traits::real::RealImpl;
use core::fmt::{Debug, Formatter};
use core::iter::{Product, Sum};
use core::ops::{Add, Div, Mul, Neg, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
#[derive(Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Complex<T: RealImpl = f64> {
    pub re: Real<T>,
    pub im: Real<T>,
}
impl<T: RealImpl> From<T> for Complex<T> {
    fn from(value: T) -> Self {
        Self::new_real(Real(value))
    }
}
impl<T: RealImpl> Debug for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match (self.re.is_zero(), self.im.is_zero()) {
            (false, false) => Debug::fmt(&(self.re, self.im), f),
            (true, false) => Debug::fmt(&self.im, f),
            (false, true) => Debug::fmt(&self.re, f),
            (true, true) => write!(f, "0.0"),
        }
    }
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
    fn sqrt(self) -> Self {
        match (self.re.is_zero(), self.im.is_zero()) {
            (true, true) => Self::default(),
            (false, true) if self.re.is_positive() => Self::new_real(self.re.sqrt()),
            (false, true) => Self::new_imag(self.re.abs().sqrt()),
            (true, false) if self.im.is_positive() => {
                let sqrt = (self.im / Real::from(2)).sqrt();
                Self::new(sqrt, sqrt)
            }
            (true, false) => {
                let sqrt = (self.im.abs() / Real::from(2)).sqrt();
                Self::new(sqrt, -sqrt)
            }
            (false, false) => {
                let abs = self.abs();
                let r = ((self.re + abs) / Real::from(2)).sqrt();
                let i = ((abs - self.re) / Real::from(2)).sqrt();
                let c = Self::new(r, i);
                if self.im.is_positive() {
                    c
                } else {
                    c.conjugate()
                }
            }
        }
    }
    fn abs(self) -> Real<T> {
        self.abs_squared().sqrt()
    }
    fn abs_squared(self) -> Real<T> {
        self.re * self.re + self.im * self.im
    }
    fn pow(self, rhs: Self) -> Self {
        if rhs.im.is_zero() {
            self.pow_real(rhs.re)
        } else if self.im.is_zero() {
            (rhs * self.re.ln()).exp()
        } else {
            (self.ln() * rhs).exp()
        }
    }
    fn pow_real(self, rhs: Self::Real) -> Self {
        if self.im.is_zero() {
            let fract = rhs.fract();
            if self.re.is_positive() || fract.is_zero() {
                return Self::new_real(RealImpl::pow(self.re, rhs));
            }
            if fract == Self::Real::from(0.5) {
                return Self::new_real(RealImpl::pow(self.re.abs(), rhs)).mul_i(false);
            }
            return (self.ln() * rhs).exp();
        }
        (self.ln() * rhs).exp()
    }
    fn root(self, rhs: Self) -> Self {
        ComplexImpl::pow(self, rhs.recip())
    }
    fn recip(self) -> Self {
        Self::new(self.re, -self.im) / self.abs_squared()
    }
    fn exp(self) -> Self {
        let (imag, real) = self.im.sin_cos();
        let e = self.re.exp();
        Self::new(real * e, imag * e)
    }
    fn ln(self) -> Self {
        Self::new(self.abs().ln(), self.arg())
    }
    fn arg(self) -> Self::Real {
        self.im.atan2(self.re)
    }
    fn sin(self) -> Self {
        let (a, b) = self.re.sin_cos();
        let (c, d) = self.im.sinh_cosh();
        Self::new(a * d, b * c)
    }
    fn cos(self) -> Self {
        let (a, b) = self.re.sin_cos();
        let (c, d) = self.im.sinh_cosh();
        Self::new(b * d, -a * c)
    }
    fn atan2(self, x: Self) -> Self {
        if self.im.is_zero() && x.im.is_zero() {
            return Self::new_real(self.re.atan2(x.re));
        }
        let d = (self * self + x * x).sqrt();
        ((self.mul_i(false) + x) / d).ln().mul_i(true)
    }
    fn mul_i(self, negative: bool) -> Self {
        if negative {
            Self::new(self.im, -self.re)
        } else {
            Self::new(-self.im, self.re)
        }
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
impl<T: RealImpl> Sum for Complex<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |total, v| total + v)
    }
}
impl<T: RealImpl> Product for Complex<T> {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::one(), |total, v| total * v)
    }
}
assign_each!(Complex<T>, Self, T: RealImpl);
