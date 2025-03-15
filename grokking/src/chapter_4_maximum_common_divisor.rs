pub fn greatest_common_divisor_recursive(first: usize, second: usize) -> usize {
    let greater = first.max(second);
    let lesser = first.min(second);
    let remainder = greater % lesser;

    if remainder == 0 {
        return lesser;
    }

    greatest_common_divisor_recursive(lesser, remainder)
}

pub fn greatest_common_divisor_simpler(first: usize, second: usize) -> usize {
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
        assert_eq!(greatest_common_divisor_recursive(1680, 640), 80);
        assert_eq!(greatest_common_divisor_simpler(1680, 640), 80);
    }
}
