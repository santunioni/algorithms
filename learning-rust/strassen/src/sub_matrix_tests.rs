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
