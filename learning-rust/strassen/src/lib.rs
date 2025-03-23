use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<usize>>,
}

type MatrixIndex = (usize, usize);

#[macro_export]
macro_rules! matrix {
    ( $( [ $( $x:expr ),* ] ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(vec![$($x),*]);
            )*
            Matrix::from_vec(temp_vec)
        }
    };
}

impl Matrix {
    pub fn zeroes(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![vec![0; cols]; rows],
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut identity = Matrix::zeroes(size, size);
        for i in 0..size {
            identity[(i, i)] = 1;
        }
        identity
    }

    pub fn from_vec(data: Vec<Vec<usize>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { rows, cols, data }
    }

    fn multiply_baseline(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.cols != other.rows {
            return Err("Matrices dimensions do not match for multiplication");
        }

        let mut result = Matrix::zeroes(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[(i, j)] += self[(i, k)] * other[(k, j)];
                }
            }
        }

        Ok(result)
    }
}

impl Index<MatrixIndex> for Matrix {
    type Output = usize;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.data[index.0][index.1] // Trocar por vetorizacação
    }
}

impl IndexMut<MatrixIndex> for Matrix {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.data[index.0][index.1] // Trocar por vetorizacação
    }
}

impl Sub<Self> for &Matrix {
    type Output = Result<Matrix, &'static str>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err("Matrices dimensions do not match");
        }
        let mut result = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(i, j)] -= rhs[(i, j)];
            }
        }
        Ok(result)
    }
}

impl Add<Self> for &Matrix {
    type Output = Result<Matrix, &'static str>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err("Matrices dimensions do not match");
        }
        let mut result = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(i, j)] += rhs[(i, j)];
            }
        }
        Ok(result)
    }
}

impl Mul<Self> for &Matrix {
    type Output = Result<Matrix, &'static str>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            return Err("Matrices dimensions do not match for multiplication");
        }

        match (self.rows, self.cols, rhs.cols) {
            (self_rows, self_cols, other_cols)
                if self_rows <= 2 || self_cols <= 2 || other_cols <= 2 =>
            {
                self.multiply_baseline(rhs)
            }
            (_self_rows, _self_cols, _other_cols) => {
                self.multiply_baseline(rhs) // Change to Strassen
            }
        }
    }
}

impl PartialEq<Self> for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if (self.rows != other.rows) || (self.cols != other.cols) {
            return false;
        }

        for row in 0..self.rows {
            for column in 0..self.cols {
                if self[(row, column)] != other[(row, column)] {
                    return false;
                }
            }
        }

        true
    }
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests {
    use crate::{matrix, Matrix};

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn should_multiply_column_matrix_by_row_matrix() -> TestResult {
        // ARRANGE
        let row_matrix = matrix!([1, 2, 5]);
        let column_matrix = matrix!([2], [3], [4]);

        // ACT
        let result = (&row_matrix * &column_matrix)?;

        // ASSERT
        assert_eq!(result, matrix![[28]]);
        Ok(())
    }

    #[test]
    fn should_multiply_row_matrix_by_column_matrix() -> TestResult {
        // ARRANGE
        let column_matrix = matrix!([2], [3], [4]);
        let row_matrix = matrix!([1, 2, 5]);

        // ACT
        let result = (&column_matrix * &row_matrix)?;

        // ASSERT
        assert_eq!(result, matrix![[2, 4, 10], [3, 6, 15], [4, 8, 20]]);
        Ok(())
    }

    #[test]
    fn should_return_same_matrix_when_multiplying_by_identity() -> TestResult {
        // ARRANGE
        let matrix = matrix!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
        let identity = Matrix::identity(3);

        // ACT
        let result = (&matrix * &identity)?;

        // ASSERT
        assert_eq!(result, matrix);
        Ok(())
    }
}
