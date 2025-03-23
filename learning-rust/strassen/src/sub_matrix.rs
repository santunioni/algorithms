use crate::matrix::Matrix;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Clone, Debug)]
pub struct SubMatrix<'a> {
    rows_window_from_parent: (usize, usize),
    cols_window_from_parent: (usize, usize),
    parent: &'a Matrix,
}

impl<'a> SubMatrix<'a> {
    pub fn new(
        matrix: &Matrix,
        rows_window: (usize, usize),
        cols_window: (usize, usize),
    ) -> SubMatrix {
        SubMatrix {
            cols_window_from_parent: cols_window,
            rows_window_from_parent: rows_window,
            parent: &matrix,
        }
    }

    fn rows(&self) -> usize {
        self.rows_window_from_parent.1 - self.rows_window_from_parent.0
    }

    fn cols(&self) -> usize {
        self.cols_window_from_parent.1 - self.cols_window_from_parent.0
    }

    fn materialize(&self) -> Matrix {
        self.parent.clone()
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

    // fn divide_in_4_parts(&'a self, at_col: usize, at_row: usize) -> [SubMatrix<'a>; 4] {
    //     //! Matrix split order is defined in the picture
    //     //! https://www.interviewbit.com/blog/wp-content/uploads/2021/12/New-quadrants-768x482.png
    //     [
    //         SubMatrix {
    //             cols: at_col,
    //             rows: at_row,
    //             data: &[],
    //         },
    //         SubMatrix {
    //             cols: self.cols - at_col,
    //             rows: at_row,
    //             data: &[],
    //         },
    //         SubMatrix {
    //             cols: at_col,
    //             rows: self.rows - at_row,
    //             data: &[],
    //         },
    //         SubMatrix {
    //             cols: self.cols - at_col,
    //             rows: self.rows - at_row,
    //             data: &[],
    //         },
    //     ]
    // }

    // fn strassen_split_with(&self, rhs: &Self) -> [SubMatrix; 8] {
    //     let a_cols = self.cols / 2;
    //     let b_cols = self.cols - a_cols;
    //     let a_rows = self.rows / 2;
    //     let c_rows = self.rows - a_rows;
    //
    //     let e_cols = rhs.cols / 2;
    //     let f_cols = rhs.cols - e_cols;
    //     []
    // }
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
