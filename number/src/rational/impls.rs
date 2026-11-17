use crate::rational::integer::IntegerTrait;
use core::ops::{Add, Div, Mul, Sub};
#[derive(Clone, Copy, Debug)]
pub struct Rational<T: IntegerTrait> {
    pub numerator: T,
    pub denominator: T,
}
impl IntegerTrait for i32 {}
impl IntegerTrait for u32 {}
impl IntegerTrait for i64 {}
impl IntegerTrait for u64 {}
impl IntegerTrait for i128 {}
impl IntegerTrait for u128 {}
impl<T: IntegerTrait> Add for Rational<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}
impl<T: IntegerTrait> Sub for Rational<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}
impl<T: IntegerTrait> Mul for Rational<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}
impl<T: IntegerTrait> Div for Rational<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
    }
}
