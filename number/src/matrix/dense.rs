use crate::matrix::size::{MatrixDimension, MatrixIndex};
use crate::traits::matrix::MatrixImpl as _;
use crate::traits::number::NumberImpl;
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::mem::needs_drop;
use core::ptr::NonNull;
use core::ptr::drop_in_place;
use core::slice;
use itertools::Itertools as _;
#[derive(Default)]
pub struct Matrix<C>
where
    C: NumberImpl,
{
    pub(crate) entries: Box<[MaybeUninit<C>]>,
    pub dimension: MatrixDimension,
}
impl<C: NumberImpl> PartialEq for Matrix<C> {
    fn eq(&self, other: &Self) -> bool {
        self.dimension == other.dimension && self.entries() == other.entries()
    }
}
impl<C: NumberImpl> Debug for Matrix<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Matrix")
            .field("dimension", &self.dimension)
            .field_with("entries", |fmt| {
                let mut list = fmt.debug_list();
                for row in self.rows() {
                    list.entry(&row);
                }
                list.finish()
            })
            .finish()
    }
}
impl<C: NumberImpl> Clone for Matrix<C> {
    fn clone(&self) -> Self {
        Self::new_with(self.dimension, |i, _| self.entries()[i])
    }
}
impl<C: NumberImpl> Drop for Matrix<C> {
    fn drop(&mut self) {
        self.drop_entries(MatrixDimension::default());
    }
}
impl<C: NumberImpl> Matrix<C> {
    pub fn new(dim: MatrixDimension) -> Self {
        Self::new_with(dim, |_, _| C::default())
    }
    pub(crate) fn new_uninit(dimension: MatrixDimension) -> Self {
        if dimension.size() == 0 {
            return Self::default();
        }
        Self {
            entries: Box::new_uninit_slice(dimension.size()),
            dimension,
        }
    }
    pub fn new_with<F>(dim: MatrixDimension, mut fun: F) -> Self
    where
        F: FnMut(usize, MatrixIndex) -> C,
    {
        let mut ret = Self::new_uninit(dim);
        for (i, entry) in ret.entries_uninit_mut().iter_mut().enumerate() {
            let index = MatrixIndex::from(dim, i).unwrap();
            entry.write(fun(i, index));
        }
        ret
    }
    pub fn new_with_try<F>(dim: MatrixDimension, mut fun: F) -> Option<Self>
    where
        F: FnMut(usize, MatrixIndex) -> Option<C>,
    {
        let mut ret = Self::new_uninit(dim);
        for (i, entry) in ret.entries_uninit_mut().iter_mut().enumerate() {
            let index = MatrixIndex::from(dim, i).unwrap();
            entry.write(fun(i, index)?);
        }
        Some(ret)
    }
    pub fn from_rows_try<F>(dim: MatrixDimension, mut fun: F) -> Option<Self>
    where
        F: FnMut(usize, MatrixIndex) -> Option<Self>,
    {
        let mut ret = Self::new_uninit(dim);
        let mut iter = ret.entries_uninit_mut().iter_mut().enumerate();
        while let Some((i, entry)) = iter.next() {
            let index = MatrixIndex::from(dim, i).unwrap();
            let vec = fun(i, index)?;
            if vec.dimension.height != 1 || vec.dimension.width != dim.width {
                return None;
            }
            entry.write(vec.entries()[0]);
            for &elem in &vec.entries()[1..] {
                let (_, e) = iter.next()?;
                e.write(elem);
            }
        }
        Some(ret)
    }
    pub fn from_cols_try<F>(dim: MatrixDimension, mut fun: F) -> Option<Self>
    where
        F: FnMut(usize, MatrixIndex) -> Option<Self>,
    {
        let mut ret = Self::new_uninit(dim);
        let mut iter = ret.entries_uninit_mut().iter_mut().enumerate();
        while let Some((i, entry)) = iter.next() {
            let index = MatrixIndex::from(dim, i).unwrap();
            let vec = fun(i, index)?;
            if vec.dimension.width != 1 || vec.dimension.height != dim.height {
                return None;
            }
            entry.write(vec.entries()[0]);
            for &elem in &vec.entries()[1..] {
                let (_, e) = iter.next()?;
                e.write(elem);
            }
        }
        ret.transpose();
        Some(ret)
    }
    pub fn new_dimension(&mut self, dim: MatrixDimension) {
        self.new_dimension_with(dim, |_, _| C::default());
    }
    pub fn new_dimension_with<F>(&mut self, dim: MatrixDimension, mut fun: F)
    where
        F: FnMut(usize, MatrixIndex) -> C,
    {
        if self.dimension == dim {
            return;
        }
        let new = Self::new_with(dim, |i, index| {
            if let Some(elem) = self.get(index) {
                elem
            } else {
                fun(i, index)
            }
        });
        self.drop_entries(dim);
        *self = new;
    }
    pub(crate) fn drop_entries(&mut self, dim: MatrixDimension) {
        for (i, entry) in self.iter_mut() {
            if dim.index(i).is_some() {
                continue;
            }
            if needs_drop::<C>() {
                unsafe {
                    drop_in_place(entry);
                }
            }
        }
        self.entries = Box::default();
        self.dimension = MatrixDimension::default();
    }
    pub(crate) fn get_uninit(&mut self, index: MatrixIndex) -> Option<&mut MaybeUninit<C>> {
        let i = self.dimension.index(index)?;
        Some(&mut self.entries[i])
    }
    pub fn entries(&self) -> &[C] {
        unsafe { self.entries.assume_init_ref() }
    }
    pub fn row(&self, n: usize) -> &[C] {
        let u = n.strict_cast::<usize>();
        &self.entries()[self.width() * u..self.width() * (u + 1)]
    }
    pub fn row_mut(&mut self, n: usize) -> &mut [C] {
        let u = n.strict_cast::<usize>();
        let width = self.width();
        &mut self.entries_mut()[width * u..width * (u + 1)]
    }
    pub fn row_disjoint_mut<const N: usize>(&mut self, n: [usize; N]) -> Option<[&mut [C]; N]> {
        if n.iter().duplicates().count() != 0 || self.entries.is_empty() {
            return None;
        }
        let u = n.map(|i| i.strict_cast::<usize>());
        let rows = u.map(|i| {
            let ptr = unsafe { self.entries.as_mut_ptr().cast::<C>().add(self.width() * i) };
            unsafe { slice::from_raw_parts_mut(ptr, self.width()) }
        });
        Some(rows)
    }
    pub fn rows(&self) -> impl Iterator<Item = &[C]> {
        self.entries().chunks_exact(self.width().max(1))
    }
    pub fn cols(&self) -> ColIter<'_, C> {
        ColIter {
            matrix: NonNull::from_ref(self),
            entry: 0,
            phantom: PhantomData,
        }
    }
    pub fn cols_mut(&mut self) -> ColMutIter<'_, C> {
        ColMutIter {
            matrix: NonNull::from_mut(self),
            entry: 0,
            phantom: PhantomData,
        }
    }
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [C]> {
        let width = self.width();
        self.entries_mut().chunks_exact_mut(width.max(1))
    }
    pub fn entries_mut(&mut self) -> &mut [C] {
        unsafe { self.entries.assume_init_mut() }
    }
    pub fn iter(&self) -> impl Iterator<Item = (MatrixIndex, C)> {
        let dim = self.dimension;
        self.entries()
            .iter()
            .copied()
            .enumerate()
            .map(move |(i, val)| (MatrixIndex::from(dim, i).unwrap(), val))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut C)> {
        let dim = self.dimension;
        self.entries_mut()
            .iter_mut()
            .enumerate()
            .map(move |(i, val)| (MatrixIndex::from(dim, i).unwrap(), val))
    }
    fn entries_uninit_mut(&mut self) -> &mut [MaybeUninit<C>] {
        &mut self.entries
    }
}
pub struct ColIter<'a, C: NumberImpl> {
    matrix: NonNull<Matrix<C>>,
    entry: usize,
    phantom: PhantomData<fn() -> &'a C>,
}
impl<'a, C: NumberImpl + 'a> Iterator for ColIter<'a, C> {
    type Item = ColInnerIter<'a, C>;
    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { self.matrix.as_ref() }.width() <= self.entry {
            return None;
        }
        let iter = ColInnerIter {
            matrix: self.matrix,
            entry: self.entry,
            phantom: PhantomData,
        };
        self.entry += 1;
        Some(iter)
    }
}
#[derive(Clone)]
pub struct ColInnerIter<'a, C: NumberImpl> {
    matrix: NonNull<Matrix<C>>,
    entry: usize,
    phantom: PhantomData<fn() -> &'a C>,
}
impl<'a, C: NumberImpl + 'a> Iterator for ColInnerIter<'a, C> {
    type Item = &'a C;
    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { self.matrix.as_ref() }.size() <= self.entry {
            return None;
        }
        let elem = &unsafe { self.matrix.as_ref() }.entries()[self.entry];
        self.entry += unsafe { self.matrix.as_ref() }.width();
        Some(elem)
    }
}
pub struct ColMutIter<'a, C: NumberImpl> {
    matrix: NonNull<Matrix<C>>,
    entry: usize,
    phantom: PhantomData<fn() -> &'a mut C>,
}
impl<'a, C: NumberImpl + 'a> Iterator for ColMutIter<'a, C> {
    type Item = ColMutInnerIter<'a, C>;
    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { self.matrix.as_ref() }.width() <= self.entry {
            return None;
        }
        let iter = ColMutInnerIter {
            matrix: self.matrix,
            entry: self.entry,
            phantom: PhantomData,
        };
        self.entry += 1;
        Some(iter)
    }
}
pub struct ColMutInnerIter<'a, C: NumberImpl> {
    matrix: NonNull<Matrix<C>>,
    entry: usize,
    phantom: PhantomData<fn() -> &'a mut C>,
}
impl<'a, C: NumberImpl + 'a> Iterator for ColMutInnerIter<'a, C> {
    type Item = &'a mut C;
    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { self.matrix.as_ref() }.size() <= self.entry {
            return None;
        }
        let elem = &mut unsafe { self.matrix.as_mut() }.entries_mut()[self.entry];
        self.entry += unsafe { self.matrix.as_ref() }.width();
        Some(elem)
    }
}
impl<C: NumberImpl, D: Into<C::NumberReal>, const N: usize, const M: usize> From<[[D; N]; M]>
    for Matrix<C>
{
    fn from(value: [[D; N]; M]) -> Self {
        let dim = MatrixDimension::new(N.strict_cast(), M.strict_cast());
        let mut vals = value.into_iter().flatten();
        Self::new_with(dim, |_, _| C::from_real(vals.next().unwrap().into()))
    }
}
