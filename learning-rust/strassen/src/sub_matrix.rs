use crate::matrix::Matrix;
use crate::sub_matrix::MatrixOperationError::{
    AdditionDimensionsDontMatch, MultiplicationDimensionsDontMatch, SubtractionDimensionsDontMatch,
};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Index, Mul, Sub};
use thiserror::Error;

pub type MatrixIndex = (usize, usize);
pub type MatrixOperationResult = Result<Matrix, MatrixOperationError>;

#[derive(Error, Debug)]
pub enum MatrixOperationError {
    MultiplicationDimensionsDontMatch,
    AdditionDimensionsDontMatch,
    SubtractionDimensionsDontMatch,
}

impl Display for MatrixOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct MatrixWindow(pub(crate) usize, pub(crate) usize);

impl MatrixWindow {
    fn size(&self) -> usize {
        1 + self.1 - self.0
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

    pub(crate) fn rows(&self) -> usize {
        self.rows_window_from_parent.size()
    }

    pub(crate) fn cols(&self) -> usize {
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
            return Err(SubtractionDimensionsDontMatch);
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
            return Err(AdditionDimensionsDontMatch);
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
    fn mult_strassen(&self, rhs: &SubMatrix) -> MatrixOperationResult {
        if self.rows() == 1 {
            let scalar: i64 = self[(0, 0)] * rhs[(0, 0)];
            return Ok(Matrix::scalar(scalar));
        }

        let half = self.rows() / 2;

        let [a, b, c, d] = &self.split_in_4_parts(half, half);
        let [e, f, g, h] = &self.split_in_4_parts(half, half);

        let p1 = a.mult_strassen(&(f - h)?.as_sub_matrix())?;
        let p2 = (a + b)?.as_sub_matrix().mult_strassen(h)?;
        let p3 = (c + d)?.as_sub_matrix().mult_strassen(e)?;
        let p4 = d.mult_strassen(&(g - e)?.as_sub_matrix())?;
        let p5 = (a + d)?
            .as_sub_matrix()
            .mult_strassen(&(e + h)?.as_sub_matrix())?;
        let p6 = (b - d)?
            .as_sub_matrix()
            .mult_strassen(&(g + h)?.as_sub_matrix())?;
        let p7 = (a - c)?
            .as_sub_matrix()
            .mult_strassen(&(e + f)?.as_sub_matrix())?;

        let [left_top, right_top, left_bottom, right_bottom] = [
            (&(&p5 + &p4)? - &(&p2 - &p6)?)?,
            (&p1 + &p2)?,
            (&p3 + &p4)?,
            (&(&p1 + &p5)? - &(&p3 + &p7)?)?,
        ];

        Ok(Matrix::assemble_from_four_pieces(
            left_top,
            right_top,
            left_bottom,
            right_bottom,
        ))
    }

    fn multiply_baseline(&self, rhs: &SubMatrix) -> Matrix {
        let mut result = Matrix::zeroes(self.rows(), rhs.cols());
        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                for k in 0..self.cols() {
                    result[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }

        result
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
            return Err(MultiplicationDimensionsDontMatch);
        }

        let left_rows = self.rows();
        let inner_multiplication_index = self.cols();
        let right_cols = rhs.cols();

        if left_rows == 0 || inner_multiplication_index == 0 || right_cols == 0 {
            return Ok(Matrix::empty());
        }

        if left_rows == 1 || inner_multiplication_index == 1 || right_cols == 1 {
            return Ok(self.multiply_baseline(&rhs));
        }

        let lesser_dimension = left_rows.min(inner_multiplication_index).min(right_cols);
        let lesser_dimension_log = lesser_dimension.ilog2();
        let dimension_to_split = 2u32.pow(lesser_dimension_log) as usize;

        let [lhs_left_top, lhs_right_top, lhs_left_bottom, lhs_right_bottom] =
            self.split_in_4_parts(dimension_to_split, dimension_to_split);

        let [rhs_left_top, rhs_right_top, rhs_left_bottom, rhs_right_bottom] =
            self.split_in_4_parts(dimension_to_split, dimension_to_split);

        let left_top =
            (&lhs_left_top.mult_strassen(&rhs_left_top)? + &(&lhs_right_top * &rhs_left_bottom)?)?;

        let right_top =
            (&(&lhs_left_top * &rhs_right_top)? + &(&lhs_right_top * &rhs_right_bottom)?)?;

        let left_bottom =
            (&(&lhs_left_bottom * &rhs_left_top)? + &(&lhs_right_bottom * &rhs_left_bottom)?)?;

        let left_right =
            (&(&lhs_left_bottom * &rhs_right_top)? + &(&lhs_right_bottom * &rhs_right_bottom)?)?;

        Ok(Matrix::assemble_from_four_pieces(
            left_top,
            right_top,
            left_bottom,
            left_right,
        ))
    }
}
