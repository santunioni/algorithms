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

    pub(crate) fn sub_matrix(
        &self,
        rows_window: MatrixWindow,
        cols_window: MatrixWindow,
    ) -> SubMatrix {
        SubMatrix::new(&self, rows_window, cols_window)
    }

    pub(crate) fn as_sub_matrix(&self) -> SubMatrix {
        self.sub_matrix(
            MatrixWindow(0, self.rows - 1),
            MatrixWindow(0, self.cols - 1),
        )
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
