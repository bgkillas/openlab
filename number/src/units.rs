use crate::assign_neg;
use crate::float::complex::Complex;
use crate::traits::assign::NegAssign;
use crate::traits::base::Operations;
use crate::traits::cast::Cast;
use crate::traits::number::NumberImpl;
use crate::traits::real::RealImpl;
use core::fmt::{Debug, Formatter};
use core::ops::Neg;
use std::simd::{Simd, SimdElement};
pub trait SimdTrait<const N: usize>: SimdElement + RealImpl {
    type Simd: Copy + Clone + Debug + PartialEq + Default + Operations + From<[Self; N]>;
}
impl<const N: usize> SimdTrait<N> for f32 {
    type Simd = Simd<Self, N>;
}
impl<const N: usize> SimdTrait<N> for f64 {
    type Simd = Simd<Self, N>;
}
#[derive(Copy, Clone, PartialEq, Default)]
pub struct NumberUnits<T: NumberImpl = Complex, U: SimdTrait<N> = f32, const N: usize = 8> {
    pub number: T,
    pub units: Units<U, N>,
    pub unit_state: UnitState<N>,
}
const _: () = assert!(size_of::<NumberUnits>() == 64);
impl<T: NumberImpl, U: SimdTrait<N>, const N: usize> NumberUnits<T, U, N> {
    pub fn new(number: T, units: Units<U, N>, unit_state: UnitState<N>) -> Self {
        Self {
            number,
            units,
            unit_state,
        }
    }
}
impl<T: NumberImpl, U: SimdTrait<N>, const N: usize> Debug for NumberUnits<T, U, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if self.units.is_zero() {
            Debug::fmt(&self.number, f)
        } else {
            Debug::fmt(&(self.number, self.units), f)
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(transparent)]
pub struct Units<U: SimdTrait<N> = f32, const N: usize = 8> {
    pub units: U::Simd,
}
#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(transparent)]
pub struct UnitState<const N: usize = 8> {
    pub state: Simd<u16, N>,
}
impl<U: SimdTrait<N>, const N: usize> From<[U; N]> for Units<U, N> {
    fn from(value: [U; N]) -> Self {
        Self {
            units: U::Simd::from(value),
        }
    }
}
impl<T: NumberImpl, U: SimdTrait<N>, const N: usize> From<T> for NumberUnits<T, U, N> {
    fn from(number: T) -> Self {
        Self::new(number, Units::default(), UnitState::default())
    }
}
impl<U: SimdTrait<N>, const N: usize> NumberImpl for Units<U, N> {
    type NumberReal = U;
    fn try_real(self) -> Option<Self::NumberReal> {
        None
    }
    fn from_real(_: Self::NumberReal) -> Self {
        unreachable!()
    }
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
    fn pow(self, rhs: Self) -> Option<Self> {
        rhs.is_zero().then_some(self)
    }
    fn root(self, rhs: Self) -> Option<Self> {
        rhs.is_zero().then_some(self)
    }
}
impl<T: NumberImpl, U: SimdTrait<N>, const N: usize> NumberImpl for NumberUnits<T, U, N>
where
    T::NumberReal: Cast<U>,
{
    type NumberReal = T::NumberReal;
    fn try_real(self) -> Option<Self::NumberReal> {
        self.number.try_real()
    }
    fn from_real(real: Self::NumberReal) -> Self {
        Self::from(T::from_real(real))
    }
    fn parse_number(base: u8, whole: u128, part: u128) -> Self {
        Self::from(T::parse_number(base, whole, part))
    }
    fn add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            number: self.number.add(rhs.number)?,
            units: self.units.add(rhs.units)?,
            unit_state: self.unit_state,
        })
    }
    fn sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            number: self.number.sub(rhs.number)?,
            units: self.units.sub(rhs.units)?,
            unit_state: self.unit_state,
        })
    }
    fn mul(self, rhs: Self) -> Option<Self> {
        Some(Self {
            number: self.number.mul(rhs.number)?,
            units: self.units.mul(rhs.units)?,
            unit_state: self.unit_state,
        })
    }
    fn div(self, rhs: Self) -> Option<Self> {
        Some(Self {
            number: self.number.div(rhs.number)?,
            units: self.units.div(rhs.units)?,
            unit_state: self.unit_state,
        })
    }
    fn pow(self, rhs: Self) -> Option<Self> {
        let mut units = self.units.pow(rhs.units)?;
        units.units *= U::Simd::from([rhs.number.try_real()?.cast(); N]);
        Some(Self {
            number: self.number.pow(rhs.number)?,
            units,
            unit_state: self.unit_state,
        })
    }
    fn root(self, rhs: Self) -> Option<Self> {
        let mut units = self.units.root(rhs.units)?;
        units.units /= U::Simd::from([rhs.number.try_real()?.cast(); N]);
        Some(Self {
            number: self.number.root(rhs.number)?,
            units,
            unit_state: self.unit_state,
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
            units: self.units,
            unit_state: self.unit_state,
        }
    }
}
assign_neg!(Units<U, N>, U: SimdTrait<N>, const N: usize);
assign_neg!(NumberUnits<T, U, N>, T: NumberImpl, U: SimdTrait<N>, const N: usize);
