use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct MatrixRef<'a> {
    rows: usize,
    cols: usize,
    data: &'a [usize],
}

impl<'a> MatrixRef<'a> {
    pub(crate) fn materialize(&self) -> Matrix {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.into(),
        }
    }
}

type MatrixIndex = (usize, usize);

impl Matrix {
    fn convert_matrix_index_to_vec_index(&self, matrix_index: MatrixIndex) -> usize {
        matrix_index.1 + matrix_index.0 * self.cols
    }

    pub fn zeroes(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0; rows * cols],
        }
    }

    pub fn get_reference(&self) -> MatrixRef {
        MatrixRef {
            rows: self.rows,
            cols: self.cols,
            data: &self.data,
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut identity = Matrix::zeroes(size, size);
        for i in 0..size {
            identity[(i, i)] = 1;
        }
        identity
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

    // fn strassen_split(&self, rhs: &Matrix) -> [Matrix; 8] {}
}

impl Index<MatrixIndex> for Matrix {
    type Output = usize;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        let vec_index = self.convert_matrix_index_to_vec_index(index);
        &self.data[vec_index]
    }
}

impl IndexMut<MatrixIndex> for Matrix {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        let vec_index = self.convert_matrix_index_to_vec_index(index);
        &mut self.data[vec_index]
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
                // //! Matrix names are defined in the picture
                // //! https://www.interviewbit.com/blog/wp-content/uploads/2021/12/New-quadrants-768x482.png
                // let [a, b, c, d, e, f, g, h] = self.stranssen_split(rhs);
                // // let p_1 =
                //
                // // Change to Strassen
                self.multiply_baseline(rhs)
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
