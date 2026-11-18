use crate::number::RationalElement;
use core::ops::Add;
impl Add for RationalElement {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        Some(match (self, rhs) {
            (Self::I64(a), Self::I64(b)) => Self::I64(a.add(b)?),
            (Self::U64(a), Self::U64(b)) => Self::U64(a.add(b)?),
            (Self::I32(a), Self::I32(b)) => Self::I32(a.add(b)?),
            (Self::U32(a), Self::U32(b)) => Self::U32(a.add(b)?),
            (Self::I128(a), Self::I128(b)) => Self::I128(a.add(b)?),
            (Self::U128(a), Self::U128(b)) => Self::U128(a.add(b)?),
            (_, _) => return None,
        })
    }
}
