use crate::complex::Complex;
use crate::matrix::Matrix;
use core::marker::PhantomData;
use core::ptr::NonNull;
pub enum Number<C>
where
    C: Complex,
{
    Complex(C),
    Matrix(Matrix<C>),
}
impl<C: Complex> Number<C> {
    pub fn get_mut(&mut self, index: usize) -> Option<&mut C> {
        match self {
            Number::Complex(val) if index == 0 => Some(val),
            Number::Matrix(mat) if index < mat.size() => mat.entries_mut().get_mut(index),
            _ => None,
        }
    }
    pub fn iter_mut(&mut self) -> ComponentIter<'_, C> {
        IntoIterator::into_iter(self)
    }
}
impl<'a, C: Complex> IntoIterator for &'a mut Number<C> {
    type Item = &'a mut C;
    type IntoIter = ComponentIter<'a, C>;
    fn into_iter(self) -> Self::IntoIter {
        ComponentIter {
            number: NonNull::from_mut(self),
            index: 0,
            phantom: PhantomData,
        }
    }
}
pub struct ComponentIter<'a, C: Complex> {
    number: NonNull<Number<C>>,
    index: usize,
    phantom: PhantomData<&'a C>,
}
impl<'a, C: Complex> Iterator for ComponentIter<'a, C> {
    type Item = &'a mut C;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = unsafe { self.number.as_mut() }.get_mut(self.index);
        self.index += 1;
        ret
    }
}
