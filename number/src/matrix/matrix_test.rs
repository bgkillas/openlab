use crate::float::complex::Complex;
use crate::float::real::Real;
use crate::matrix::dense::Matrix;
use crate::matrix::size::MatrixDimension;
use crate::traits::complex::ComplexImpl;
use crate::traits::matrix::MatrixImpl;
#[test]
pub fn matrix_allocation() {
    let dim1 = MatrixDimension::new(3, 5);
    let dim2 = MatrixDimension::new(4, 2);
    let mut matrix = Matrix::new_with(dim1, |_, i| {
        Complex::new(Real(i.row as f64), Real(i.col as f64))
    });
    for (i, val) in matrix.iter_mut() {
        assert_eq!(&Complex::new(Real(i.row as f64), Real(i.col as f64)), val);
    }
    matrix.new_dimension_with(dim2, |_, i| {
        Complex::new(Real(-(i.row as f64)), Real(-(i.col as f64)))
    });
    for (i, val) in matrix.iter_mut() {
        let n = if dim1.index(i).is_some() {
            Complex::new(Real(i.row as f64), Real(i.col as f64))
        } else {
            Complex::new(Real(-(i.row as f64)), Real(-(i.col as f64)))
        };
        assert_eq!(&n, val);
    }
}
#[test]
pub fn matrix_multiplication() {
    let mut a: Matrix<Complex<f64>> = Matrix::from([[1, 2, 4, 2], [7, 4, 3, 2], [2, 7, 5, 2]]);
    let b: Matrix<Complex<f64>> = Matrix::from([[1, 5, 4], [7, 4, 7], [2, 7, 1], [7, 5, 3]]);
    let c: Matrix<Complex<f64>> = Matrix::from([[37, 51, 28], [55, 82, 65], [75, 83, 68]]);
    a.mul(&b);
    assert_eq!(a, c);
}
#[test]
pub fn matrix_transpose() {
    let a: Matrix<Complex<f64>> = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let mut b: Matrix<Complex<f64>> = Matrix::from([[1, 4, 7], [2, 5, 8], [3, 6, 9]]);
    b.transpose();
    assert_eq!(a, b);
    let a: Matrix<Complex<f64>> = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    let mut b: Matrix<Complex<f64>> = Matrix::from([[1, 5, 9], [2, 6, 10], [3, 7, 11], [4, 8, 12]]);
    b.transpose();
    assert_eq!(a, b);
    let a: Matrix<Complex<f64>> = Matrix::from([[1, 5, 9], [2, 6, 10], [3, 7, 11], [4, 8, 12]]);
    let mut b: Matrix<Complex<f64>> = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    b.transpose();
    assert_eq!(a, b);
}
