use crate::base::BaseImpl;
use crate::complex::ComplexImpl;
pub trait RealImpl: BaseImpl + Copy {
    type Complex: ComplexImpl;
    #[must_use]
    fn sqrt(self) -> Self;
}
