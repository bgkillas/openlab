use crate::float::complex::Complex;
use crate::float::real::Real;
use crate::traits::assign::NegAssign;
use crate::traits::complex::ComplexImpl as _;
use crate::traits::real::RealImpl;
use core::fmt::Debug;
use core::ops::Neg;
pub trait NumberImpl:
    Default + Copy + Clone + Debug + PartialEq + Neg<Output = Self> + NegAssign
{
    fn zero() -> Self {
        Self::default()
    }
    fn one() -> Self {
        Self::parse_number(2, 1, 0)
    }
    fn is_zero(self) -> bool {
        self == Self::zero()
    }
    fn parse_number(base: u8, whole: u128, part: u128) -> Self;
    fn add(self, rhs: Self) -> Option<Self>;
    fn sub(self, rhs: Self) -> Option<Self>;
    fn mul(self, rhs: Self) -> Option<Self>;
    fn div(self, rhs: Self) -> Option<Self>;
    fn add_assign(&mut self, rhs: Self) -> Option<()> {
        *self = self.add(rhs)?;
        Some(())
    }
    fn sub_assign(&mut self, rhs: Self) -> Option<()> {
        *self = self.sub(rhs)?;
        Some(())
    }
    fn mul_assign(&mut self, rhs: Self) -> Option<()> {
        *self = self.mul(rhs)?;
        Some(())
    }
    fn div_assign(&mut self, rhs: Self) -> Option<()> {
        *self = self.div(rhs)?;
        Some(())
    }
    fn sum(mut iter: impl Iterator<Item = Option<Self>>) -> Option<Self> {
        let Some(maybe_sum) = iter.next() else {
            return Some(Self::default());
        };
        let mut sum = maybe_sum?;
        for next in iter {
            sum.add_assign(next?)?;
        }
        Some(sum)
    }
}
impl<T: RealImpl> NumberImpl for Complex<T> {
    fn parse_number(base: u8, whole: u128, part: u128) -> Self {
        Self::new_real(Real::<T>::parse_number(base, whole, part))
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
}
impl<T: RealImpl> NumberImpl for Real<T> {
    fn parse_number(base: u8, whole: u128, part: u128) -> Self {
        Self(T::parse_number(base, whole, part))
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
}
