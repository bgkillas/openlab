use crate::complex::Complex;
use crate::matrix::{Matrix, MatrixDimension};
use core::marker::PhantomData;
use core::ptr::NonNull;
#[derive(Clone)]
pub enum Number<C>
where
    C: Complex,
{
    Complex(C),
    Matrix(Matrix<C>),
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NumberDimension {
    #[default]
    Complex,
    Matrix(MatrixDimension),
}
#[derive(Clone, Copy, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NumberIndex {
    pub index: usize,
    pub dimension: NumberDimension,
}
impl<C: Complex> Number<C> {
    pub fn new(dimension: NumberDimension) -> Self {
        match dimension {
            NumberDimension::Complex => Self::Complex(C::default()),
            NumberDimension::Matrix(dim) => Self::Matrix(Matrix::new(dim)),
        }
    }
    pub fn size(&self) -> NumberDimension {
        match self {
            Self::Complex(_) => NumberDimension::Complex,
            Self::Matrix(mat) => NumberDimension::Matrix(mat.dimension),
        }
    }
    pub fn get_checked(&self, index: NumberIndex) -> Option<&C> {
        if index.dimension != self.size() {
            return None;
        }
        match self {
            Self::Complex(val) if index.index == 0 => Some(val),
            Self::Matrix(mat) if index.index < mat.size() => mat.entries().get(index.index),
            _ => None,
        }
    }
    pub fn get(&self, index: usize) -> Option<&C> {
        match self {
            Self::Complex(val) if index == 0 => Some(val),
            Self::Matrix(mat) if index < mat.size() => mat.entries().get(index),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut C> {
        match self {
            Self::Complex(val) if index == 0 => Some(val),
            Self::Matrix(mat) if index < mat.size() => mat.entries_mut().get_mut(index),
            _ => None,
        }
    }
    pub fn iter(&self) -> ComponentIter<'_, C> {
        ComponentIter {
            number: NonNull::from_ref(self),
            index: 0,
            phantom: PhantomData,
        }
    }
    pub fn iter_mut(&mut self) -> ComponentIterMut<'_, C> {
        ComponentIterMut {
            number: NonNull::from_mut(self),
            index: 0,
            phantom: PhantomData,
        }
    }
    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (NumberIndex, &mut C)> {
        let dimension = self.size();
        self.iter_mut()
            .enumerate()
            .map(move |(index, val)| (NumberIndex { index, dimension }, val))
    }
}
impl<'a, C: Complex> IntoIterator for &'a Number<C> {
    type Item = &'a C;
    type IntoIter = ComponentIter<'a, C>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, C: Complex> IntoIterator for &'a mut Number<C> {
    type Item = &'a mut C;
    type IntoIter = ComponentIterMut<'a, C>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
pub struct ComponentIter<'a, C: Complex> {
    number: NonNull<Number<C>>,
    index: usize,
    phantom: PhantomData<&'a C>,
}
impl<'a, C: Complex> Iterator for ComponentIter<'a, C> {
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = unsafe { self.number.as_ref() }.get(self.index);
        self.index += 1;
        ret
    }
}
pub struct ComponentIterMut<'a, C: Complex> {
    number: NonNull<Number<C>>,
    index: usize,
    phantom: PhantomData<&'a mut C>,
}
impl<'a, C: Complex> Iterator for ComponentIterMut<'a, C> {
    type Item = &'a mut C;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = unsafe { self.number.as_mut() }.get_mut(self.index);
        self.index += 1;
        ret
    }
}
