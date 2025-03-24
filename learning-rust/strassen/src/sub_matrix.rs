use crate::matrix::Matrix;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Clone, Debug)]
pub struct MatrixWindow(pub(crate) usize, pub(crate) usize);

impl MatrixWindow {
    fn size(&self) -> usize {
        1 + self.1 - self.0
    }
}

impl Add<Self> for MatrixWindow {
    type Output = MatrixWindow;

    fn add(self, rhs: Self) -> Self::Output {
        MatrixWindow(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SubMatrix<'a> {
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
        let rows = self.rows();
        let cols = self.cols();

        if rows == 0 || cols == 0 {
            return Matrix::empty();
        }

        let mut matrix = Matrix::zeroes(rows, cols);
        for row in 0..rows {
            for col in 0..cols {
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

    pub(crate) fn split_horizontally(&self, at_col: usize) -> (SubMatrix<'a>, SubMatrix<'a>) {
        let cols = self.cols();

        let left = SubMatrix {
            rows_window_from_parent: self.rows_window_from_parent.clone(),
            cols_window_from_parent: MatrixWindow(0, at_col - 1),
            parent: self.parent,
        };
        let right = SubMatrix {
            rows_window_from_parent: self.rows_window_from_parent.clone(),
            cols_window_from_parent: MatrixWindow(at_col, cols - 1),
            parent: self.parent,
        };

        (left, right)
    }

    pub(crate) fn split_vertically(&self, at_row: usize) -> (SubMatrix<'a>, SubMatrix<'a>) {
        let rows = self.rows();

        let top = SubMatrix {
            rows_window_from_parent: MatrixWindow(0, at_row - 1),
            cols_window_from_parent: self.cols_window_from_parent.clone(),
            parent: self.parent,
        };
        let bottom = SubMatrix {
            rows_window_from_parent: MatrixWindow(at_row, rows - 1),
            cols_window_from_parent: self.cols_window_from_parent.clone(),
            parent: self.parent,
        };

        (top, bottom)
    }

    pub(crate) fn split_in_4_parts(&'a self, at_row: usize, at_col: usize) -> [SubMatrix<'a>; 4] {
        //! Matrix split order is defined in the picture
        //! https://www.interviewbit.com/blog/wp-content/uploads/2021/12/New-quadrants-768x482.png
        let (left, right) = self.split_horizontally(at_col);
        let ((left_top, left_bottom), (right_top, right_bottom)) = (
            left.split_vertically(at_row),
            right.split_vertically(at_row),
        );
        [left_top, right_top, left_bottom, right_bottom]
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
