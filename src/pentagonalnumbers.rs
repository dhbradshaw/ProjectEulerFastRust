pub fn is_pentagonal(n: u64) -> bool {
    let discriminant = 1 + 24 * n;
    (1f64 + (discriminant as f64).sqrt())/6f64 % 1f64 == 0f64
}

pub fn pentagonal(n: u64) -> u64 {
    let num = n * (3 * n - 1);
    num / 2
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_pentagonal() {
        // 1, 5, 12, 22, 35, 51, 70, 92, 117, 145
        assert_eq!(is_pentagonal(1), true);
        assert_eq!(is_pentagonal(2), false);

        assert_eq!(is_pentagonal(4), false);
        assert_eq!(is_pentagonal(5), true);
        assert_eq!(is_pentagonal(6), false);

        assert_eq!(is_pentagonal(11), false);
        assert_eq!(is_pentagonal(12), true);
        assert_eq!(is_pentagonal(13), false);

        assert_eq!(is_pentagonal(21), false);
        assert_eq!(is_pentagonal(22), true);
        assert_eq!(is_pentagonal(23), false);

        assert_eq!(is_pentagonal(34), false);
        assert_eq!(is_pentagonal(35), true);
        assert_eq!(is_pentagonal(36), false);

        assert_eq!(is_pentagonal(50), false);
        assert_eq!(is_pentagonal(51), true);
        assert_eq!(is_pentagonal(52), false);

        assert_eq!(is_pentagonal(69), false);
        assert_eq!(is_pentagonal(70), true);
        assert_eq!(is_pentagonal(71), false);

        assert_eq!(is_pentagonal(91), false);
        assert_eq!(is_pentagonal(92), true);
        assert_eq!(is_pentagonal(93), false);

        assert_eq!(is_pentagonal(116), false);
        assert_eq!(is_pentagonal(117), true);
        assert_eq!(is_pentagonal(118), false);

        assert_eq!(is_pentagonal(144), false);
        assert_eq!(is_pentagonal(145), true);
        assert_eq!(is_pentagonal(146), false);


    }
}
