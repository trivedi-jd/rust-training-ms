pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert_eq!(is_even(24), true);
    }

    #[test]
    fn is_false_when_odd() {
        assert_eq!(is_even(31), false);
    }
}