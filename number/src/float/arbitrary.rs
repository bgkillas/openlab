use core::ops::{Add, Div, Mul, Sub};
use malachite::base::num::logic::traits::SignificantBits as _;
use malachite::float::BigExponentFloat;
#[derive(Clone, Debug)]
pub struct Float {
    pub float: BigExponentFloat,
    pub prec: u64,
}
impl Mul for Float {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.with_prec(rhs, |a, b, _| a * b)
    }
}
impl Sub for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.with_prec(rhs, |a, b, _| a - b)
    }
}
impl Add for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.with_prec(rhs, |a, b, _| a + b)
    }
}
impl Div for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.with_prec(rhs, |a, b, p| a.div_prec(b, p).0)
    }
}
impl Float {
    pub fn max_prec(&self, other: &Self) -> u64 {
        self.prec.max(other.prec)
    }
    pub fn max_bits(&self, other: &Self) -> u64 {
        self.bits().max(other.bits())
    }
    pub fn additional_prec(&self, other: &Self) -> u64 {
        self.max_prec(other) + self.max_bits(other)
    }
    pub fn bits(&self) -> u64 {
        self.float.significant_bits()
    }
    #[must_use]
    pub fn with_prec<F>(self, rhs: Self, f: F) -> Self
    where
        F: FnOnce(BigExponentFloat, BigExponentFloat, u64) -> BigExponentFloat,
    {
        let prec = self.max_prec(&rhs);
        let additional = self.additional_prec(&rhs);
        Self {
            float: f(self.float, rhs.float, additional),
            prec,
        }
    }
}
