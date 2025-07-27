use crate::matrix::Matrix;

/// Computes the tensor product of two matrices.
/// 
/// # Arguments
/// * m1: Matrix 1
/// * m2: Matrix 2
fn tensor_product(m1: Matrix, m2: Matrix) -> Matrix {
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
}
