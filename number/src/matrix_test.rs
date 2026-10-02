use crate::matrix::{Matrix, MatrixDimension};
use std::num::Complex;
#[test]
pub fn matrix_allocation() {
    let dim1 = MatrixDimension::new(3, 5);
    let dim2 = MatrixDimension::new(4, 2);
    let mut matrix = Matrix::new_with(dim1, |_, i| Complex::<f64>::new(i.row as f64, i.col as f64));
    for (i, val) in matrix.entries_enumerate_mut() {
        assert_eq!(&Complex::<f64>::new(i.row as f64, i.col as f64), val);
    }
    matrix.new_dimension_with(dim2, |_, i| {
        Complex::<f64>::new(-(i.row as f64), -(i.col as f64))
    });
    for (i, val) in matrix.entries_enumerate_mut() {
        let n = if dim1.index(i).is_some() {
            Complex::<f64>::new(i.row as f64, i.col as f64)
        } else {
            Complex::<f64>::new(-(i.row as f64), -(i.col as f64))
        };
        assert_eq!(&n, val);
    }
}
