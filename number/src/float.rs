use crate::base::BaseImpl;
use crate::complex::ComplexImpl;
use crate::real::RealImpl;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}
impl BaseImpl for f64 {}
impl BaseImpl for Complex<f64> {}
impl RealImpl for f64 {}
impl ComplexImpl for Complex<f64> {}
impl<T: RealImpl> Complex<T> {
    #[must_use]
    pub const fn new(re: T, im: T) -> Complex<T> {
        Complex { re, im }
    }
    #[must_use]
    pub fn conjugate(self) -> Self {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
}
impl<T: RealImpl> Neg for Complex<T> {
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex::new(-self.re, -self.im)
    }
}
impl<T: RealImpl> Add<Self> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}
impl<T: RealImpl> Add<T> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: T) -> Self::Output {
        Complex::new(self.re + rhs, self.im)
    }
}
impl<T: RealImpl> Sub<Self> for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}
impl<T: RealImpl> Sub<T> for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: T) -> Self::Output {
        Complex::new(self.re - rhs, self.im)
    }
}
impl<T: RealImpl> AddAssign for Complex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}
impl<T: RealImpl> SubAssign for Complex<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}
impl MulAssign for Complex<f64> {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}
impl DivAssign for Complex<f64> {
    fn div_assign(&mut self, rhs: Self) {
        todo!()
    }
}
macro_rules! impl_complex_mul_div {
    ($ty:ty) => {
        impl Mul for Complex<$ty> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let std::num::Complex { re, im } = std::num::Complex::new(self.re, self.im)
                    * std::num::Complex::new(rhs.re, rhs.im);
                Self { re, im }
            }
        }
        impl Div for Complex<$ty> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let std::num::Complex { re, im } = std::num::Complex::new(self.re, self.im)
                    / std::num::Complex::new(rhs.re, rhs.im);
                Self { re, im }
            }
        }
    };
}
impl_complex_mul_div!(f16);
impl_complex_mul_div!(f32);
impl_complex_mul_div!(f64);
impl_complex_mul_div!(f128);
