use crate::traits::assign::NegAssign;
use core::fmt::Debug;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
pub trait BaseImpl:
    Default
    + Copy
    + Clone
    + Debug
    + PartialEq
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Neg<Output = Self>
    + NegAssign
    + Sum<Self>
    + Product<Self>
{
    fn zero() -> Self {
        Self::default()
    }
    fn one() -> Self;
    fn is_zero(self) -> bool;
    fn parse_number(base: u8, whole: u128, part: u128) -> Self;
}
