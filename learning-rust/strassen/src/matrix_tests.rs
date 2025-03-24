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
