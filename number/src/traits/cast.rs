use crate::float::real::Real;
use crate::traits::real::RealImpl;
pub trait Cast<To>: Sized {
    fn cast(self) -> To;
}
macro_rules! impl_float_to_float {
    ($Float:ty => $($Flt:ty),+) => {
        $(
            impl Cast<$Flt> for $Float {
                #[inline]
                #[expect(clippy::allow_attributes)]
                #[allow(clippy::cast_possible_truncation)]
                fn cast(self) -> $Flt {
                    self as $Flt
                }
            }
        )+
    }
}
impl_float_to_float!(f16 => f16, f32, f64);
impl_float_to_float!(f32 => f16, f32, f64);
impl_float_to_float!(f64 => f16, f32, f64);
impl<R: Cast<T> + RealImpl, T> Cast<T> for Real<R> {
    fn cast(self) -> T {
        self.0.cast()
    }
}
