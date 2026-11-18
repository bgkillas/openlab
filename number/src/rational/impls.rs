use crate::rational::integer::IntegerTrait;
use core::ops::{Add, Div, Mul, Sub};
#[derive(Clone, Copy, Debug)]
pub struct Rational<T: IntegerTrait> {
    pub numerator: T,
    pub denominator: T,
}
macro_rules! impl_int {
    ($ty:ty) => {
        impl IntegerTrait for $ty {
            fn add(self, rhs: Self) -> Option<Self> {
                self.checked_add(rhs)
            }
            fn sub(self, rhs: Self) -> Option<Self> {
                self.checked_sub(rhs)
            }
            fn mul(self, rhs: Self) -> Option<Self> {
                self.checked_mul(rhs)
            }
            fn div(self, rhs: Self) -> Option<Self> {
                self.checked_div(rhs)
            }
        }
    };
}
impl_int!(i32);
impl_int!(u32);
impl_int!(i64);
impl_int!(u64);
impl_int!(i128);
impl_int!(u128);
impl<T: IntegerTrait> Add for Rational<T> {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        Some(Self {
            numerator: self
                .numerator
                .mul(rhs.denominator)?
                .add(rhs.numerator.mul(self.denominator)?)?,
            denominator: self.denominator.mul(rhs.denominator)?,
        })
    }
}
impl<T: IntegerTrait> Sub for Rational<T> {
    type Output = Option<Self>;
    fn sub(self, rhs: Self) -> Self::Output {
        Some(Self {
            numerator: self
                .numerator
                .mul(rhs.denominator)?
                .sub(rhs.numerator.mul(self.denominator)?)?,
            denominator: self.denominator.mul(rhs.denominator)?,
        })
    }
}
impl<T: IntegerTrait> Mul for Rational<T> {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Self::Output {
        Some(Self {
            numerator: self.numerator.mul(rhs.numerator)?,
            denominator: self.denominator.mul(rhs.denominator)?,
        })
    }
}
impl<T: IntegerTrait> Div for Rational<T> {
    type Output = Option<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        Some(Self {
            numerator: self.numerator.mul(rhs.denominator)?,
            denominator: self.denominator.mul(rhs.numerator)?,
        })
    }
}
