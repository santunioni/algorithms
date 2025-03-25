use crate::matrix::Matrix;
use crate::sub_matrix::MatrixOperationError::{
    AdditionDimensionsDontMatch, MultiplicationDimensionsDontMatch, SubtractionDimensionsDontMatch,
};
use crate::sub_matrix::SubMatrix::Empty;
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
pub(crate) enum SubMatrix<'a> {
    Empty,
    Filled {
        rows_window_from_parent: MatrixWindow,
        cols_window_from_parent: MatrixWindow,
        parent: &'a Matrix,
    },
}

impl<'a> SubMatrix<'a> {
    pub(crate) fn rows(&self) -> usize {
        match self {
            SubMatrix::Empty => 0,
            SubMatrix::Filled {
                rows_window_from_parent,
                ..
            } => rows_window_from_parent.size(),
        }
    }

    pub(crate) fn cols(&self) -> usize {
        match self {
            SubMatrix::Empty => 0,
            SubMatrix::Filled {
                cols_window_from_parent,
                ..
            } => cols_window_from_parent.size(),
        }
    }

    pub(crate) fn materialize(&self) -> Matrix {
        let rows = self.rows();
        let cols = self.cols();

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
        match self {
            SubMatrix::Empty => panic!("Trying to index empty Sub Matrix"),
            SubMatrix::Filled {
                parent,
                rows_window_from_parent,
                cols_window_from_parent,
            } => {
                let row = index.0 + rows_window_from_parent.0;
                let column = index.1 + cols_window_from_parent.0;
                &parent[(row, column)]
            }
        }
    }
}

impl<'a> Mul<i64> for &SubMatrix<'a> {
    type Output = Matrix;

    fn mul(self, rhs: i64) -> Self::Output {
        let rows = self.rows();
        let cols = self.cols();
        let mut result = self.materialize();
        for i in 0..rows {
            for j in 0..cols {
                result[(i, j)] *= rhs;
            }
        }
        result
    }
}

impl<'a> Mul<&SubMatrix<'a>> for i64 {
    type Output = Matrix;

    fn mul(self, rhs: &SubMatrix) -> Self::Output {
        rhs * self
    }
}

impl<'a> Sub<Self> for &SubMatrix<'a> {
    type Output = MatrixOperationResult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SubMatrix::Empty, SubMatrix::Filled { .. }) => Ok(-1 * rhs),
            (SubMatrix::Filled { .. }, SubMatrix::Empty) => Ok(self.materialize()),
            (SubMatrix::Empty, SubMatrix::Empty) => Ok(Matrix::empty()),
            (SubMatrix::Filled { .. }, SubMatrix::Filled { .. }) => {
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
    }
}

impl<'a> Add<Self> for &SubMatrix<'a> {
    type Output = MatrixOperationResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (SubMatrix::Empty, SubMatrix::Filled { .. }) => Ok(rhs.materialize()),
            (SubMatrix::Filled { .. }, SubMatrix::Empty) => Ok(self.materialize()),
            (SubMatrix::Empty, SubMatrix::Empty) => Ok(Matrix::empty()),
            (SubMatrix::Filled { .. }, SubMatrix::Filled { .. }) => {
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
    }
}

impl<'a> SubMatrix<'a> {
    #[allow(dead_code)]
    fn mult_strassen(&self, rhs: &SubMatrix) -> MatrixOperationResult {
        match (self, rhs) {
            (SubMatrix::Empty, SubMatrix::Filled { .. }) => Ok(Matrix::empty()),
            (SubMatrix::Filled { .. }, SubMatrix::Empty) => Ok(Matrix::empty()),
            (SubMatrix::Empty, SubMatrix::Empty) => Ok(Matrix::empty()),
            (SubMatrix::Filled { .. }, SubMatrix::Filled { .. }) => {
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
        }
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

    pub(crate) fn split_horizontally(&self, at_col: usize) -> [SubMatrix<'a>; 2] {
        match self {
            SubMatrix::Empty => [SubMatrix::Empty, SubMatrix::Empty],
            SubMatrix::Filled {
                rows_window_from_parent,
                parent,
                ..
            } => {
                let cols = self.cols();

                let left = if at_col >= 1 {
                    SubMatrix::Filled {
                        rows_window_from_parent: rows_window_from_parent.clone(),
                        cols_window_from_parent: MatrixWindow(0, at_col - 1),
                        parent,
                    }
                } else {
                    SubMatrix::Empty
                };

                let right = if cols >= at_col + 1 {
                    SubMatrix::Filled {
                        rows_window_from_parent: rows_window_from_parent.clone(),
                        cols_window_from_parent: MatrixWindow(at_col, cols - 1),
                        parent,
                    }
                } else {
                    SubMatrix::Empty
                };

                [left, right]
            }
        }
    }

    pub(crate) fn split_vertically(&self, at_row: usize) -> [SubMatrix<'a>; 2] {
        match self {
            SubMatrix::Empty => [SubMatrix::Empty, SubMatrix::Empty],
            SubMatrix::Filled {
                cols_window_from_parent,
                parent,
                ..
            } => {
                let rows = self.rows();

                let top = if at_row >= 1 {
                    SubMatrix::Filled {
                        rows_window_from_parent: MatrixWindow(0, at_row - 1),
                        cols_window_from_parent: cols_window_from_parent.clone(),
                        parent,
                    }
                } else {
                    SubMatrix::Empty
                };

                let bottom = if rows >= at_row + 1 {
                    SubMatrix::Filled {
                        rows_window_from_parent: MatrixWindow(at_row, rows - 1),
                        cols_window_from_parent: cols_window_from_parent.clone(),
                        parent,
                    }
                } else {
                    SubMatrix::Empty
                };

                [top, bottom]
            }
        }
    }

    pub(crate) fn split_in_4_parts(&'a self, at_row: usize, at_col: usize) -> [SubMatrix<'a>; 4] {
        //! Matrix split order is defined in the picture
        //! https://www.interviewbit.com/blog/wp-content/uploads/2021/12/New-quadrants-768x482.png
        let [left, right] = self.split_horizontally(at_col);
        let [[left_top, left_bottom], [right_top, right_bottom]] = [
            left.split_vertically(at_row),
            right.split_vertically(at_row),
        ];
        [left_top, right_top, left_bottom, right_bottom]
    }
}

impl<'a> Mul<Self> for &SubMatrix<'a> {
    type Output = MatrixOperationResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Empty, _) => return Ok(Matrix::empty()),
            (_, Empty) => return Ok(Matrix::empty()),
            (_, _) => {}
        }

        if self.cols() != rhs.rows() {
            return Err(MultiplicationDimensionsDontMatch);
        }

        return Ok(self.multiply_baseline(rhs));

        // let left_rows = self.rows();
        // let inner_multiplication_index = self.cols();
        // let right_cols = rhs.cols();
        //
        // if left_rows == 1 || inner_multiplication_index == 1 || right_cols == 1 {
        //     return Ok(self.multiply_baseline(&rhs));
        // }
        //
        // if left_rows == 0 || inner_multiplication_index == 0 || right_cols == 0 {
        //     return Ok(Matrix::empty());
        // }
        //
        // let lesser_dimension = left_rows.min(inner_multiplication_index).min(right_cols);
        // let lesser_dimension_log = lesser_dimension.ilog2();
        // let dimension_to_split = 2u32.pow(lesser_dimension_log) as usize;
        //
        // if left_rows == dimension_to_split
        //     && inner_multiplication_index == dimension_to_split
        //     && right_cols == dimension_to_split
        // {
        //     return Ok(self.multiply_baseline(rhs));
        // }
        //
        // let [lhs_left_top, lhs_right_top, lhs_left_bot, lhs_right_bot] =
        //     self.split_in_4_parts(dimension_to_split, dimension_to_split);
        //
        // let [rhs_left_top, rhs_right_top, rhs_left_bot, rhs_right_bot] =
        //     rhs.split_in_4_parts(dimension_to_split, dimension_to_split);
        //
        // let left_top = (&(&lhs_left_top * &rhs_left_top)? + &(&lhs_right_top * &rhs_left_bot)?)?;
        //
        // let right_top = (&(&lhs_left_top * &rhs_right_top)? + &(&lhs_right_top * &rhs_right_bot)?)?;
        //
        // let left_bottom = (&(&lhs_left_bot * &rhs_left_top)? + &(&lhs_right_bot * &rhs_left_bot)?)?;
        //
        // let left_right =
        //     (&(&lhs_left_bot * &rhs_right_top)? + &(&lhs_right_bot * &rhs_right_bot)?)?;
        //
        // Ok(Matrix::assemble_from_four_pieces(
        //     left_top,
        //     right_top,
        //     left_bottom,
        //     left_right,
        // ))
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;
    use crate::matrix::Matrix;
    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn should_divide_3_by_3_matrix_in_4_parts() -> TestResult {
        let original_matrix: Matrix = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let original_matrix_as_sub_matrix = original_matrix.as_sub_matrix();

        let [a, b, c, d] = original_matrix_as_sub_matrix
            .split_in_4_parts(2, 2)
            .map(|v| v.materialize());

        assert_eq!(a, matrix![[1, 2], [4, 5]]);
        assert_eq!(b, matrix![[3], [6]]);
        assert_eq!(c, matrix![[7, 8]]);
        assert_eq!(d, matrix![[9]]);
        Ok(())
    }

    #[test]
    fn should_divide_4_by_4_matrix_in_4_parts() -> TestResult {
        let original_matrix: Matrix = matrix![
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16]
        ];
        let original_matrix_as_sub_matrix = original_matrix.as_sub_matrix();

        let [top_left, top_right, bottom_left, bottom_right] = original_matrix_as_sub_matrix
            .split_in_4_parts(2, 2)
            .map(|v| v.materialize());

        assert_eq!(top_left, matrix![[1, 2], [5, 6]]);
        assert_eq!(top_right, matrix![[3, 4], [7, 8]]);
        assert_eq!(bottom_left, matrix![[9, 10], [13, 14]]);
        assert_eq!(bottom_right, matrix![[11, 12], [15, 16]]);
        Ok(())
    }
}
