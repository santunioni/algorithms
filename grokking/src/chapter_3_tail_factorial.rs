use tailcall::tailcall;

#[allow(unused_macros)]
macro_rules! time_it {
    ($expr:expr) => {{
        let start = std::time::Instant::now();
        let iterations = 300;
        for _ in 1..iterations {
            $expr;
        }
        let result = $expr;
        (result, start.elapsed() / iterations)
    }};
}

#[tailcall]
fn inner_factorial_tail(num: u128, current: u128) -> u128 {
    if num == 1 {
        current
    } else {
        inner_factorial_tail(num - 1, num * current)
    }
}

fn factorial_with_tail(num: u128) -> u128 {
    inner_factorial_tail(num, 1)
}

fn factorial_without_tail(num: u128) -> u128 {
    if num == 1 {
        num
    } else {
        num * factorial_without_tail(num - 1)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::assert_lt;

    #[test]
    fn should_return_factorial() {
        let number = 34;

        let (factorial_with_tail_result, factorial_with_tail_time) =
            time_it!(factorial_with_tail(number));

        let (factorial_without_tail_result, factorial_without_tail_time) =
            time_it!(factorial_without_tail(number));

        assert_eq!(factorial_with_tail_result, factorial_without_tail_result);
        assert_lt!(factorial_with_tail_time, factorial_without_tail_time);
    }
}
