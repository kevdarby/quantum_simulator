use core::panic;

use crate::matrix::Matrix;

/// Computes the tensor product of two matrices.
/// 
/// # Arguments
/// * m1: Matrix 1
/// * m2: Matrix 2
pub fn tensor_product(m1: Matrix, m2: Matrix) -> Matrix {
    let rows1 = m1.m.len();
    let cols1 = m1.m[0].len();
    let rows2 = m2.m.len();
    let cols2 = m2.m[0].len();

    let mut m = vec![vec![0.0; cols1 * cols2]; rows1 * rows2];

    for i in 0..rows1 {
        for j in 0..cols1 {
            for k in 0..rows2 {
                for l in 0..cols2 {
                    m[i * rows2 + k][j * cols2 + l] = m1.m[i][j] * m2.m[k][l];
                }
            }
        }
    }

    Matrix::new(m)
}

pub fn dot_product(m1: Matrix, m2: Matrix) -> Matrix {
    let rows1 = m1.m.len();
    let cols1 = m1.m[0].len();
    let rows2 = m2.m.len();
    let cols2 = m2.m[0].len();

    if cols1 != rows2 {
        panic!("The number of columns in m1 ({cols1}) must match the number of rows in m2 ({rows2}).");
    }

    let mut m = vec![vec![0.0; cols2]; rows1];

    for i in 0..rows1 {
        for j in 0..cols2 {
            for k in 0..cols1 {
                m[i][j] += m1.m[i][k] * m2.m[k][j];
            }
        }
    }

    Matrix { m }
}


pub fn transpose(m: Matrix) -> Matrix {
    let rows = m.m.len();
    let cols = m.m[0].len();
    let mut transposed = vec![vec![0.0; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = m.m[i][j];
        }
    }

    Matrix::new(transposed)
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_tensor_product_identity_hadamard() {
        let identity = Matrix::new(vec![
            vec![1.0, 0.0],
            vec![0.0, 1.0],
        ]);

        let hadamard = Matrix::new(vec![
            vec![1.0, 1.0],
            vec![1.0, -1.0],
        ]);

        let result = tensor_product(identity, hadamard);

        let expected = Matrix::new(vec![
            vec![1.0, 1.0, 0.0, 0.0],
            vec![1.0, -1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 1.0],
            vec![0.0, 0.0, 1.0, -1.0],
        ]);

        assert_eq!(result.m, expected.m);
    }

    #[test]
    fn test_dot_product_basic() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ]);

        let b = Matrix::new(vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ]);

        let result = dot_product(a, b);

        let expected = Matrix::new(vec![
            vec![19.0, 22.0],
            vec![43.0, 50.0],
        ]);

        assert_eq!(result.m, expected.m);
    }

    #[test]
    fn test_dot_product_non_square() {
        let a = Matrix::new(vec![
            vec![2.0, 3.0, 4.0],
        ]);

        let b = Matrix::new(vec![
            vec![1.0],
            vec![0.0],
            vec![1.0],
        ]);

        let result = dot_product(a, b);

        let expected = Matrix::new(vec![
            vec![6.0],
        ]);

        assert_eq!(result.m, expected.m);
    }
}
