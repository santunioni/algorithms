#[cfg(test)]
mod tests {
    use crate::matrix;
    use crate::matrix::Matrix;
    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn should_divide_3_by_3_matrix_in_4_parts() -> TestResult {
        let original_matrix: Matrix = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let original_matrix_as_sub_matrix = original_matrix.as_sub_matrix();

        let [a, b, c, d] = original_matrix_as_sub_matrix.divide_in_4_parts(2, 2);
        let [a, b, c, d] = [
            a.materialize(),
            b.materialize(),
            c.materialize(),
            d.materialize(),
        ];

        assert_eq!(a, matrix![[1, 2], [4, 5]]);
        assert_eq!(b, matrix![[3], [6]]);
        assert_eq!(c, matrix![[7, 8]]);
        assert_eq!(d, matrix![[9]]);
        Ok(())
    }
}
