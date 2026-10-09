use crate::assign_neg;
use crate::traits::assign::NegAssign;
use crate::traits::base::Operations;
use crate::traits::number::NumberImpl;
use crate::traits::real::RealImpl;
use core::fmt::Debug;
use core::ops::Neg;
use std::simd::{Simd, SimdElement};
pub trait SimdTrait<const N: usize>: SimdElement + RealImpl {
    type Simd: Copy + Clone + Debug + PartialEq + Default + Operations;
}
impl<const N: usize> SimdTrait<N> for f16 {
    type Simd = Simd<Self, N>;
}
impl<const N: usize> SimdTrait<N> for f32 {
    type Simd = Simd<Self, N>;
}
impl<const N: usize> SimdTrait<N> for f64 {
    type Simd = Simd<Self, N>;
}
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct NumberUnits<T: NumberImpl, U: SimdTrait<N>, const N: usize> {
    pub number: T,
    pub units: Units<U, N>,
}
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Units<U: SimdTrait<N>, const N: usize> {
    pub units: U::Simd,
}
impl<T: NumberImpl, U: SimdTrait<N>, const N: usize> From<T> for NumberUnits<T, U, N> {
    fn from(number: T) -> Self {
        Self {
            number,
            units: Units::default(),
        }
    }
}
impl<U: SimdTrait<N>, const N: usize> NumberImpl for Units<U, N> {
    fn parse_number(_: u8, _: u128, _: u128) -> Self {
        Self::default()
    }
    fn add(self, rhs: Self) -> Option<Self> {
        (self.units == rhs.units).then_some(self)
    }
    fn sub(self, rhs: Self) -> Option<Self> {
        (self.units == rhs.units).then_some(self)
    }
    fn mul(mut self, rhs: Self) -> Option<Self> {
        self.units += rhs.units;
        Some(self)
    }
    fn div(mut self, rhs: Self) -> Option<Self> {
        self.units -= rhs.units;
        Some(self)
    }
}
impl<T: NumberImpl, U: SimdTrait<N>, const N: usize> NumberImpl for NumberUnits<T, U, N> {
    fn parse_number(base: u8, whole: u128, part: u128) -> Self {
        Self {
            number: T::parse_number(base, whole, part),
            units: Units::parse_number(base, whole, part),
        }
    }
    fn add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            number: self.number.add(rhs.number)?,
            units: self.units.add(rhs.units)?,
        })
    }
    fn sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            number: self.number.sub(rhs.number)?,
            units: self.units.sub(rhs.units)?,
        })
    }
    fn mul(self, rhs: Self) -> Option<Self> {
        Some(Self {
            number: self.number.mul(rhs.number)?,
            units: self.units.mul(rhs.units)?,
        })
    }
    fn div(self, rhs: Self) -> Option<Self> {
        Some(Self {
            number: self.number.div(rhs.number)?,
            units: self.units.div(rhs.units)?,
        })
    }
}
impl<U: SimdTrait<N>, const N: usize> Neg for Units<U, N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self
    }
}
impl<T: NumberImpl, U: SimdTrait<N>, const N: usize> Neg for NumberUnits<T, U, N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            number: -self.number,
            units: -self.units,
        }
    }
}
assign_neg!(Units<U, N>, U: SimdTrait<N>, const N: usize);
assign_neg!(NumberUnits<T, U, N>, T: NumberImpl, U: SimdTrait<N>, const N: usize);
