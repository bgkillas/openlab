use crate::traits::base::BaseImpl;
use crate::traits::complex::ComplexImpl;
pub trait RealImpl: BaseImpl {
    type Complex: ComplexImpl;
    #[must_use]
    fn sqrt(self) -> Self;
}
