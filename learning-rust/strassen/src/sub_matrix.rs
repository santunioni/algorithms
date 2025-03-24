use crate::matrix::Matrix;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Clone, Debug)]
pub struct MatrixWindow(pub usize, pub usize);

impl MatrixWindow {
    fn size(&self) -> usize {
        self.1 - self.0 + 1
    }
}

impl Add<Self> for MatrixWindow {
    type Output = MatrixWindow;

    fn add(self, rhs: Self) -> Self::Output {
        MatrixWindow(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Debug)]
pub struct SubMatrix<'a> {
    rows_window_from_parent: MatrixWindow,
    cols_window_from_parent: MatrixWindow,
    parent: &'a Matrix,
}

impl<'a> SubMatrix<'a> {
    pub fn new(matrix: &Matrix, rows_window: MatrixWindow, cols_window: MatrixWindow) -> SubMatrix {
        SubMatrix {
            cols_window_from_parent: cols_window,
            rows_window_from_parent: rows_window,
            parent: &matrix,
        }
    }

    fn rows(&self) -> usize {
        self.rows_window_from_parent.size()
    }

    fn cols(&self) -> usize {
        self.cols_window_from_parent.size()
    }

    pub(crate) fn materialize(&self) -> Matrix {
        let mut matrix = Matrix::zeroes(self.rows(), self.cols());
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                matrix[(row, col)] = self[(row, col)]
            }
        }
        matrix
    }
}

impl<'a> Index<MatrixIndex> for SubMatrix<'a> {
    type Output = i64;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        let row = index.0 + self.rows_window_from_parent.0;
        let column = index.1 + self.cols_window_from_parent.0;
        &self.parent[(row, column)]
    }
}

impl<'a> Sub<Self> for &SubMatrix<'a> {
    type Output = MatrixOperationResult;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            return Err("Matrices dimensions do not match");
        }
        let mut result = self.materialize();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result[(i, j)] -= rhs[(i, j)];
            }
        }
        Ok(result)
    }
}

impl<'a> Add<Self> for &SubMatrix<'a> {
    type Output = MatrixOperationResult;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            return Err("Matrices dimensions do not match");
        }
        let mut result = self.materialize();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result[(i, j)] += rhs[(i, j)];
            }
        }
        Ok(result)
    }
}

impl<'a> SubMatrix<'a> {
    fn multiply_baseline(&self, rhs: &SubMatrix) -> MatrixOperationResult {
        if self.cols() != rhs.rows() {
            return Err("Matrices dimensions do not match for multiplication");
        }

        let mut result = Matrix::zeroes(self.rows(), rhs.cols());
        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                for k in 0..self.cols() {
                    result[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }

        Ok(result)
    }

    pub(crate) fn divide_in_4_parts(
        &'a self,
        first_matrix_finishes_at_row: usize,
        first_matrix_finishes_at_col: usize,
    ) -> [SubMatrix<'a>; 4] {
        //! Matrix split order is defined in the picture
        //! https://www.interviewbit.com/blog/wp-content/uploads/2021/12/New-quadrants-768x482.png
        let rows = self.rows();
        let cols = self.cols();

        let a = SubMatrix {
            rows_window_from_parent: MatrixWindow(0, first_matrix_finishes_at_row - 1),
            cols_window_from_parent: MatrixWindow(0, first_matrix_finishes_at_col - 1),
            parent: self.parent,
        };
        let b = SubMatrix {
            rows_window_from_parent: MatrixWindow(0, first_matrix_finishes_at_row - 1),
            cols_window_from_parent: MatrixWindow(first_matrix_finishes_at_col, cols - 1),
            parent: self.parent,
        };
        let c = SubMatrix {
            rows_window_from_parent: MatrixWindow(first_matrix_finishes_at_row, rows - 1),
            cols_window_from_parent: MatrixWindow(0, first_matrix_finishes_at_col - 1),
            parent: self.parent,
        };
        let d = SubMatrix {
            rows_window_from_parent: MatrixWindow(first_matrix_finishes_at_row, rows - 1),
            cols_window_from_parent: MatrixWindow(first_matrix_finishes_at_col, cols - 1),
            parent: self.parent,
        };

        [a, b, c, d]
    }
}

impl<'a> Mul<Self> for &SubMatrix<'a> {
    type Output = MatrixOperationResult;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols() != rhs.rows() {
            return Err("Matrices dimensions do not match for multiplication");
        }

        match (self.rows(), self.cols(), rhs.cols()) {
            (self_rows, self_cols, other_cols)
                if self_rows <= 2 || self_cols <= 2 || other_cols <= 2 =>
            {
                self.multiply_baseline(&rhs)
            }
            (_self_rows, _self_cols, _other_cols) => {
                // Matrix names are defined in the picture
                // https://www.interviewbit.com/blog/wp-content/uploads/2021/12/New-quadrants-768x482.png
                // let [a, b, c, d, e, f, g, h] = self.strassen_split(rhs);
                // // let p_1 =
                //
                // // Change to Strassen
                self.multiply_baseline(&rhs)
            }
        }
    }
}

pub type MatrixIndex = (usize, usize);
pub type MatrixOperationResult = Result<Matrix, &'static str>;
