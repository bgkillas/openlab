use crate::float::complex::Complex;
use crate::traits::assign::NegAssign;
use crate::traits::real::RealImpl;
use core::fmt::Debug;
use core::ops::Neg;
pub trait NumberImpl:
    Default + Copy + Clone + Debug + PartialEq + Neg<Output = Self> + NegAssign
{
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
}
impl<T: RealImpl> NumberImpl for Complex<T> {
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
