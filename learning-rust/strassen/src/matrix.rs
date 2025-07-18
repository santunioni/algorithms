use crate::sub_matrix::{MatrixIndex, MatrixOperationResult, MatrixWindow, SubMatrix};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<i64>,
}

impl Matrix {
    pub fn zeroes(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0; rows * cols],
        }
    }

    pub fn scalar(value: i64) -> Self {
        let mut matrix = Matrix::zeroes(1, 1);
        matrix[(0, 0)] = value;
        matrix
    }

    pub fn empty() -> Self {
        Matrix {
            rows: 0,
            cols: 0,
            data: vec![],
        }
    }

    pub(crate) fn assemble_from_four_pieces(
        left_top: Matrix,
        right_top: Matrix,
        left_bottom: Matrix,
        right_bottom: Matrix,
    ) -> Matrix {
        let (top_rows, bottom_rows) = (left_top.rows, left_bottom.rows);
        let (left_columns, right_columns) = (left_top.cols, right_top.cols);
        let (total_rows, total_cols) = (top_rows + bottom_rows, left_columns + right_columns);

        let mut matrix = Matrix::zeroes(total_rows, total_cols);

        for row in 0..top_rows {
            for col in 0..left_columns {
                matrix[(row, col)] = left_top[(row, col)];
            }
            for col in left_columns..total_cols {
                matrix[(row, col)] = right_top[(row, col - left_columns)];
            }
        }
        for row in top_rows..total_rows {
            for col in 0..left_columns {
                matrix[(row, col)] = left_bottom[(row - top_rows, col)];
            }
            for col in left_columns..total_cols {
                matrix[(row, col)] = right_bottom[(row - top_rows, col - left_columns)];
            }
        }

        matrix
    }

    pub fn identity(size: usize) -> Self {
        let mut identity = Matrix::zeroes(size, size);
        for i in 0..size {
            identity[(i, i)] = 1;
        }
        identity
    }

    pub(crate) fn as_sub_matrix(&self) -> SubMatrix {
        if self.rows == 0 || self.cols == 0 {
            SubMatrix::Empty
        } else {
            SubMatrix::Filled {
                cols_window_from_parent: MatrixWindow(0, self.cols - 1),
                rows_window_from_parent: MatrixWindow(0, self.rows - 1),
                parent: self,
            }
        }
    }
}

impl Index<MatrixIndex> for Matrix {
    type Output = i64;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        let vec_index = index.1 + index.0 * self.cols;
        &self.data[vec_index]
    }
}

impl IndexMut<MatrixIndex> for Matrix {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        let vec_index = index.1 + index.0 * self.cols;
        &mut self.data[vec_index]
    }
}

impl Add<Self> for &Matrix {
    type Output = MatrixOperationResult;

    fn add(self, rhs: Self) -> Self::Output {
        &self.as_sub_matrix() + &rhs.as_sub_matrix()
    }
}

impl Sub<Self> for &Matrix {
    type Output = MatrixOperationResult;

    fn sub(self, rhs: Self) -> Self::Output {
        &self.as_sub_matrix() - &rhs.as_sub_matrix()
    }
}

impl Mul<Self> for &Matrix {
    type Output = MatrixOperationResult;

    fn mul(self, rhs: Self) -> Self::Output {
        &self.as_sub_matrix() * &rhs.as_sub_matrix()
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

#[macro_export]
macro_rules! matrix {
    ( $( [ $( $x:expr ),* ] ),* ) => {
        {
            let temp_vec = vec![$( vec![$($x),*] ),*];
            let rows = temp_vec.len();
            let cols = temp_vec[0].len();
            let mut matrix = Matrix::zeroes(rows, cols);
            for (row_idx, row_vec) in temp_vec.iter().enumerate() {
                for (col_index, &val) in row_vec.iter().enumerate() {
                    matrix[(row_idx, col_index)] = val;
                }
            }
            matrix
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::matrix;
    use crate::matrix::Matrix;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn should_create_matrix_with_macro() -> TestResult {
        let matrix = matrix![[1, 2], [3, 4]];

        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(0, 1)], 2);
        assert_eq!(matrix[(1, 0)], 3);
        assert_eq!(matrix[(1, 1)], 4);
        Ok(())
    }

    #[test]
    fn should_multiply_column_matrix_by_row_matrix() -> TestResult {
        // ARRANGE
        let row_matrix = matrix!([1, 2, 5]);
        let column_matrix = matrix!([2], [3], [4]);

        // ACT
        let result = &row_matrix * &column_matrix;

        // ASSERT
        assert_eq!(result?, matrix![[28]]);
        Ok(())
    }

    #[test]
    fn should_multiply_row_matrix_by_column_matrix() -> TestResult {
        // ARRANGE
        let column_matrix = matrix!([2], [3], [4]);
        let row_matrix = matrix!([1, 2, 5]);

        // ACT
        let result = &column_matrix * &row_matrix;

        // ASSERT
        assert_eq!(result?, matrix![[2, 4, 10], [3, 6, 15], [4, 8, 20]]);
        Ok(())
    }

    #[test]
    fn should_return_same_matrix_when_multiplying_by_identity() -> TestResult {
        // ARRANGE
        let matrix = matrix!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
        let identity = Matrix::identity(3);

        // ACT
        let result = &matrix * &identity;

        // ASSERT
        assert_eq!(result?, matrix);
        Ok(())
    }

    #[test]
    fn should_return_identity() -> TestResult {
        // ASSERT
        let size = 30;
        assert_eq!(
            (&Matrix::identity(size) * &Matrix::identity(size))?,
            Matrix::identity(size)
        );
        Ok(())
    }

    #[test]
    fn should_add_matrices() -> TestResult {
        // ARRANGE
        let matrix1 = matrix!([1, 2], [3, 4]);
        let matrix2 = matrix!([5, 6], [7, 8]);

        // ACT
        let result = &matrix1 + &matrix2;

        // ASSERT
        let expected = matrix!([6, 8], [10, 12]);
        assert_eq!(result?, expected);
        Ok(())
    }

    #[test]
    fn should_subtract_matrices() -> TestResult {
        // ARRANGE
        let matrix1 = matrix!([5, 6], [7, 8]);
        let matrix2 = matrix!([1, 2], [3, 4]);

        // ACT
        let result = &matrix1 - &matrix2;

        // ASSERT
        let expected = matrix!([4, 4], [4, 4]);
        assert_eq!(result?, expected);
        Ok(())
    }

    #[test]
    fn should_add_matrices_with_different_dimensions() -> TestResult {
        // ARRANGE
        let matrix1 = matrix!([1, 2], [3, 4]);
        let matrix2 = matrix!([5, 6, 7]);

        // ACT
        let result = &matrix1 + &matrix2;

        // ASSERT
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn should_subtract_matrices_with_different_dimensions() -> TestResult {
        // ARRANGE
        let matrix1 = matrix!([1, 2], [3, 4]);
        let matrix2 = matrix!([5, 6, 7]);

        // ACT
        let result = &matrix1 - &matrix2;

        // ASSERT
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn should_multiply_matrices_with_invalid_dimensions() -> TestResult {
        // ARRANGE
        let matrix1 = matrix!([1, 2]);
        let matrix2 = matrix!([3, 4]);

        // ACT
        let result = &matrix1 * &matrix2;

        // ASSERT
        assert!(result.is_err());
        Ok(())
    }
}
