use tailcall::tailcall;

#[tailcall]
fn inner_factorial_tail(num: u128, current: u128) -> u128 {
    if num <= 1 {
        current
    } else {
        inner_factorial_tail(num - 1, num * current)
    }
}

pub fn factorial_with_tail(num: u128) -> u128 {
    inner_factorial_tail(num, 1)
}

pub fn factorial_without_tail(num: u128) -> u128 {
    if num <= 1 {
        num
    } else {
        num * factorial_without_tail(num - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_factorial() {
        let number = 34;
        assert_eq!(factorial_with_tail(number), factorial_without_tail(number));
    }
}
