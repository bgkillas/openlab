use crate::number::{FloatElement, RationalElement};
use core::ops::{Add, Div, Mul, Sub};
macro_rules! impl_op {
    ($ty:ty, $tr:ident, $tr_fun:ident, $($d:ident,)*) => {
        impl $tr for $ty {
            type Output = Option<Self>;
            fn $tr_fun(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    $(
                        (Self::$d(a), Self::$d(b)) => Some(Self::$d($tr::$tr_fun(a, b)?)),
                    )*
                    (_, _) => None,
                }
            }
        }
    };
}
impl_op!(RationalElement, Add, add, I64, U64, I32, U32, I128, U128,);
impl_op!(RationalElement, Sub, sub, I64, U64, I32, U32, I128, U128,);
impl_op!(RationalElement, Mul, mul, I64, U64, I32, U32, I128, U128,);
impl_op!(RationalElement, Div, div, I64, U64, I32, U32, I128, U128,);
impl_op!(FloatElement, Add, add, F64, F32,);
impl_op!(FloatElement, Sub, sub, F64, F32,);
impl_op!(FloatElement, Mul, mul, F64, F32,);
impl_op!(FloatElement, Div, div, F64, F32,);
