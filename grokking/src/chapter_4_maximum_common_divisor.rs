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
    fn should_return_80() {
        assert_eq!(greatest_common_divisor_simpler_but_slow(1680, 640), 80);
        assert_eq!(greatest_common_divisor_loop_euclides(1680, 640), 80);
        assert_eq!(greatest_common_divisor_simpler_but_slow(1680, 640), 80);
    }
}
