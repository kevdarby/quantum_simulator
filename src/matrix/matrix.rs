use std::fmt;

pub struct Matrix {
    pub m: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(mat: Vec<Vec<f64>>) -> Self {
        let new_matrix = Matrix { m: mat };

        if !new_matrix.check_valid() {
            panic!("Invalid Matrix! All rows must have the same length.");
        }

        new_matrix
    }

    fn check_valid(&self) -> bool {
        if self.m.len() == 0 || self.m[0].len() == 0 {
            return false
        }
        let row_length = self.m[0].len();
        for row in &self.m {
            if row.len() != row_length {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.m {
            for val in row {
                write!(f, "{:.2} ", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_matrix(){
        Matrix::new(
            vec!(
                vec![0.0, 1.0],
                vec![1.0, 0.0],
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_unequal_row_length_matrix(){
        Matrix::new(
            vec!(
                vec![0.0, 1.0, 3.0],
                vec![1.0, 0.0],
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_empty_matrix_completely() {
        Matrix::new(vec![]);
    }

    #[test]
    #[should_panic]
    fn test_empty_matrix_with_empty_row() {
        Matrix::new(vec![vec![]]);
    }
}