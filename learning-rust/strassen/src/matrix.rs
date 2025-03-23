use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<i64>,
}

#[derive(Clone, Debug)]
struct SubMatrix<'a> {
    rows_window_from_parent: (usize, usize),
    cols_window_from_parent: (usize, usize),
    parent: &'a Matrix,
}

impl<'a> SubMatrix<'a> {
    fn rows(&self) -> usize {
        self.rows_window_from_parent.1 - self.rows_window_from_parent.0
    }

    fn cols(&self) -> usize {
        self.cols_window_from_parent.1 - self.cols_window_from_parent.0
    }
}

impl<'a> From<&'a SubMatrix<'a>> for Matrix {
    fn from(value: &'a SubMatrix<'a>) -> Self {
        value.parent.clone()
    }
}

impl<'a> From<&'a Matrix> for SubMatrix<'a> {
    fn from(value: &'a Matrix) -> Self {
        SubMatrix {
            rows_window_from_parent: (0, value.rows),
            cols_window_from_parent: (0, value.cols),
            parent: &value,
        }
    }
}

type MatrixIndex = (usize, usize);

impl Matrix {
    pub fn zeroes(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0; rows * cols],
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut identity = Matrix::zeroes(size, size);
        for i in 0..size {
            identity[(i, i)] = 1;
        }
        identity
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

type MatrixOperationResult = Result<Matrix, &'static str>;

impl<'a> Sub<Self> for &SubMatrix<'a> {
    type Output = MatrixOperationResult;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            return Err("Matrices dimensions do not match");
        }
        let mut result = Matrix::from(self);
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
        let mut result = Matrix::from(self);
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
                // let [a, b, c, d, e, f, g, h] = self.stranssen_split(rhs);
                // // let p_1 =
                //
                // // Change to Strassen
                self.multiply_baseline(&rhs)
            }
        }
    }
}

impl Add<Self> for &Matrix {
    type Output = MatrixOperationResult;

    fn add(self, rhs: Self) -> Self::Output {
        &SubMatrix::from(self) + &SubMatrix::from(rhs)
    }
}

impl Sub<Self> for &Matrix {
    type Output = MatrixOperationResult;

    fn sub(self, rhs: Self) -> Self::Output {
        &SubMatrix::from(self) - &SubMatrix::from(rhs)
    }
}

impl Mul<Self> for &Matrix {
    type Output = MatrixOperationResult;

    fn mul(self, rhs: Self) -> Self::Output {
        &SubMatrix::from(self) * &SubMatrix::from(rhs)
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
