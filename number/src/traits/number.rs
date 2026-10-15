use crate::float::complex::Complex;
use crate::float::real::Real;
use crate::traits::assign::NegAssign;
use crate::traits::complex::ComplexImpl;
use crate::traits::real::RealImpl;
use core::fmt::Debug;
use core::ops::Neg;
pub trait NumberImpl:
    Default + Copy + Clone + Debug + PartialEq + Neg<Output = Self> + NegAssign
{
    type NumberReal: RealImpl;
    fn zero() -> Self {
        Self::default()
    }
    fn one() -> Self {
        Self::parse_number(2, 1, 0)
    }
    fn try_real(self) -> Option<Self::NumberReal>;
    fn from_real(real: Self::NumberReal) -> Self;
    fn is_zero(self) -> bool {
        self == Self::zero()
    }
    fn parse_number(base: u8, whole: u128, part: u128) -> Self;
    fn add(self, rhs: Self) -> Option<Self>;
    fn sub(self, rhs: Self) -> Option<Self>;
    fn mul(self, rhs: Self) -> Option<Self>;
    fn div(self, rhs: Self) -> Option<Self>;
    fn pow(self, rhs: Self) -> Option<Self>;
    fn root(self, rhs: Self) -> Option<Self>;
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
    fn pow_assign(&mut self, rhs: Self) -> Option<()> {
        *self = self.pow(rhs)?;
        Some(())
    }
    fn root_assign(&mut self, rhs: Self) -> Option<()> {
        *self = self.root(rhs)?;
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
    type NumberReal = Real<T>;
    fn try_real(self) -> Option<Self::NumberReal> {
        self.im.is_zero().then_some(self.re)
    }
    fn from_real(real: Self::NumberReal) -> Self {
        Self::new_real(real)
    }
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
    fn pow(self, rhs: Self) -> Option<Self> {
        Some(ComplexImpl::pow(self, rhs))
    }
    fn root(self, rhs: Self) -> Option<Self> {
        Some(ComplexImpl::root(self, rhs))
    }
}
impl<T: RealImpl> NumberImpl for Real<T> {
    type NumberReal = Self;
    fn try_real(self) -> Option<Self::NumberReal> {
        Some(self)
    }
    fn from_real(real: Self::NumberReal) -> Self {
        real
    }
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
    fn pow(self, rhs: Self) -> Option<Self> {
        Some(RealImpl::pow(self, rhs))
    }
    fn root(self, rhs: Self) -> Option<Self> {
        Some(RealImpl::root(self, rhs))
    }
}
