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
        let prec = self.max_prec(&rhs);
        Self {
            float: self.float * rhs.float,
            prec,
        }
    }
}
impl Sub for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let prec = self.max_prec(&rhs);
        Self {
            float: self.float - rhs.float,
            prec,
        }
    }
}
impl Add for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let prec = self.max_prec(&rhs);
        Self {
            float: self.float + rhs.float,
            prec,
        }
    }
}
impl Div for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let (prec, compute) = self.new_prec(&rhs);
        Self {
            float: self.float.div_prec(rhs.float, compute).0,
            prec,
        }
    }
}
impl Float {
    pub fn max_prec(&self, other: &Self) -> u64 {
        self.prec.max(other.prec)
    }
    pub fn new_prec(&self, other: &Self) -> (u64, u64) {
        let prec = self.max_prec(other);
        (prec, prec + self.bits() + other.bits())
    }
    pub fn bits(&self) -> u64 {
        self.float.significant_bits()
    }
}
