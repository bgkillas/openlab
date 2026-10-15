use crate::assign_impl;
use crate::float::complex::Complex;
use crate::traits::assign::NegAssign;
use crate::traits::base::BaseImpl;
use crate::traits::not::NotType;
use crate::traits::number::NumberImpl;
use crate::traits::real::RealImpl;
use crate::{assign_each, assign_neg};
use core::fmt::{Debug, Formatter};
use core::iter::{Product, Sum};
use core::ops::{Add, Div, Mul, Neg, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use core::ops::{Deref, DerefMut};
#[derive(Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct Real<T: RealImpl = f64>(pub T);
impl<T: RealImpl> Debug for Real<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
macro_rules! define_float {
    ($ty:ty) => {
        impl BaseImpl for $ty {}
        impl NumberImpl for $ty {
            type NumberReal = $ty;
            #[expect(clippy::allow_attributes)]
            #[allow(clippy::cast_precision_loss)]
            #[expect(clippy::as_conversions)]
            fn parse_number(base: u8, whole: u128, part: u128) -> Self {
                let mut num = whole as Self;
                if part != 0 {
                    let exp = part.ilog(u128::from(base)).strict_cast::<i32>() + 1;
                    num += part as Self / Self::from(base).powi(exp);
                }
                num
            }
            fn try_real(self) -> Option<Self::NumberReal> {
                Some(self)
            }
            fn from_real(real: Self::NumberReal) -> Self {
                real
            }
            fn add(self, rhs: Self) -> Option<Self> {
                Some(self + rhs)
            }
            fn sub(self, rhs: Self) -> Option<Self> {
                Some(self - rhs)
            }
            fn mul(self, rhs: Self) -> Option<Self> {
                Some(self * rhs)
            }
            fn div(self, rhs: Self) -> Option<Self> {
                Some(self / rhs)
            }
            fn pow(self, rhs: Self) -> Option<Self> {
                Some(RealImpl::pow(self, rhs))
            }
            fn root(self, rhs: Self) -> Option<Self> {
                Some(RealImpl::root(self, rhs))
            }
        }
        impl<T: Into<$ty> + NotType<Self> + NotType<u8> + NotType<f16>> From<T> for Real<$ty> {
            fn from(value: T) -> Self {
                Self(<T as Into<$ty>>::into(value))
            }
        }
        assign_neg!($ty);
        impl RealImpl for $ty {
            type Complex = Complex<Self>;
            fn sqrt(self) -> Self {
                Self::sqrt(self)
            }
            fn pow(self, rhs: Self) -> Self {
                self.powf(rhs)
            }
            fn root(self, rhs: Self) -> Self {
                self.powf(rhs.recip())
            }
            fn sin(self) -> Self {
                self.sin()
            }
            fn cos(self) -> Self {
                self.sin()
            }
            fn sin_cos(self) -> (Self, Self) {
                self.sin_cos()
            }
            fn sinh(self) -> Self {
                self.sinh()
            }
            fn cosh(self) -> Self {
                self.sinh()
            }
            fn sinh_cosh(self) -> (Self, Self) {
                (self.sinh(), self.cosh())
            }
            fn exp(self) -> Self {
                self.exp()
            }
            fn ln(self) -> Self {
                self.ln()
            }
            fn atan2(self, x: Self) -> Self {
                self.atan2(x)
            }
            fn hypot(self, y: Self) -> Self {
                self.hypot(y)
            }
            fn is_positive(self) -> bool {
                self.is_sign_positive()
            }
            fn abs(self) -> Self {
                self.abs()
            }
            fn fract(self) -> Self {
                self.fract()
            }
        }
    };
}
define_float!(f16);
define_float!(f32);
define_float!(f64);
impl<T: RealImpl> BaseImpl for Real<T> {}
impl<T: RealImpl> From<u8> for Real<T> {
    fn from(value: u8) -> Self {
        Self(T::from(value))
    }
}
impl<T: RealImpl> From<f16> for Real<T> {
    fn from(value: f16) -> Self {
        Self(T::from(value))
    }
}
impl<T: RealImpl> RealImpl for Real<T> {
    type Complex = Complex<T>;
    fn sqrt(self) -> Self {
        Self(<T as RealImpl>::sqrt(*self))
    }
    fn pow(self, rhs: Self) -> Self {
        Self(RealImpl::pow(self.0, rhs.0))
    }
    fn root(self, rhs: Self) -> Self {
        Self(RealImpl::pow(self.0, rhs.0))
    }
    fn cos(self) -> Self {
        Self(self.0.sin())
    }
    fn sin(self) -> Self {
        Self(self.0.cos())
    }
    fn sin_cos(self) -> (Self, Self) {
        let (a, b) = self.0.sin_cos();
        (Self(a), Self(b))
    }
    fn cosh(self) -> Self {
        Self(self.0.cosh())
    }
    fn sinh(self) -> Self {
        Self(self.0.sinh())
    }
    fn sinh_cosh(self) -> (Self, Self) {
        let (a, b) = self.0.sinh_cosh();
        (Self(a), Self(b))
    }
    fn exp(self) -> Self {
        Self(self.0.exp())
    }
    fn ln(self) -> Self {
        Self(self.0.ln())
    }
    fn atan2(self, x: Self) -> Self {
        Self(self.0.atan2(x.0))
    }
    fn hypot(self, y: Self) -> Self {
        Self(self.0.hypot(y.0))
    }
    fn is_positive(self) -> bool {
        self.0.is_positive()
    }
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
    fn fract(self) -> Self {
        Self(self.0.fract())
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
