pub fn greatest_common_divisor_recursive_euclides(first: usize, second: usize) -> usize {
    let greater = first.max(second);
    let lesser = first.min(second);
    let remainder = greater % lesser;

    if remainder == 0 {
        return lesser;
    }

    greatest_common_divisor_recursive_euclides(lesser, remainder)
}

pub fn greatest_common_divisor_loop_euclides(first: usize, second: usize) -> usize {
    let mut greater = first.max(second);
    let mut lesser = first.min(second);

    loop {
        let remainder = greater % lesser;
        if remainder == 0 {
            return lesser;
        }
        greater = lesser;
        lesser = remainder;
    }
}

pub fn greatest_common_divisor_simpler_but_slow(first: usize, second: usize) -> usize {
    for trial in (1..=first.min(second)).rev() {
        if first % trial == 0 && second % trial == 0 {
            return trial;
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_expected_gcd() {
        for example in [
            (1680, 640, 80),
            (640, 1680, 80),
            (1680, 1680, 1680),
            (1, 2, 1),
            (3, 5, 1),
            (3, 6, 3),
            (6, 3, 3),
            (1, 1, 1),
        ] {
            let (first, second, expected) = example;
            assert_eq!(
                greatest_common_divisor_recursive_euclides(first, second),
                expected
            );
            assert_eq!(
                greatest_common_divisor_loop_euclides(first, second),
                expected
            );
            assert_eq!(
                greatest_common_divisor_simpler_but_slow(first, second),
                expected
            );
        }
    }
}
