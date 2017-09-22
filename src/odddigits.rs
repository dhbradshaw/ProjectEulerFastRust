pub fn highest_even_digit(n: usize) -> Option<(usize, usize)> {
    let mut nc = n;
    if n == 0 {
        return Some((0, 1));
    }
    let mut place = 1;
    let mut highest_place = 0;
    let mut highest_even_digit = 1;
    while nc > 0 {
        let last = nc % 10;
        if last % 2 == 0 {
            highest_place = place;
            highest_even_digit = last;
        }
        nc = nc / 10;
        place += 1;
    }
    if highest_place == 0 {
        return None;
    }
    Some((highest_even_digit, highest_place))
}

pub fn next_odd_digit_number(n: usize) -> usize {
    match highest_even_digit(n) {
        Some((digit, place)) => {
            // Increment the even digit, fill rest with ones.
            let mut nc = n / 10usize.pow(place as u32);
            nc *= 10;
            nc += digit + 1;
            for _ in 0..(place - 1) {
                nc *= 10;
                nc += 1;
            }
            nc
        }
        None => {
            // Try adding two.  If no even digit, return.  Else, increment the even digit, fill rest with ones
            let nc = n + 2;
            match highest_even_digit(nc) {
                Some(_) => next_odd_digit_number(nc),
                None => nc,
            }
        }
    }
}

pub fn next_odd_sans_five_digit(d: u32) -> u32 {
    match d {
        9 => 1,
        8 => 9,
        7 => 9,
        6 => 7,
        5 => 7,
        4 => 7,
        3 => 7,
        2 => 3,
        1 => 3,
        0 => 1,
        _ => 0,
    }
}

pub fn next_odd_sans_five(n: u32) -> u32 {
    // Doesn't find hidden even digits.
    // But never creates them, so if you just use this you're golden.
    let mut nc = n;
    let mut last = nc % 10;
    match last {
        9 => {
            while last == 9 {
                nc /= 10;
                last = nc % 10;
            }
            nc -= last;
            nc += next_odd_sans_five_digit(last);
            while nc < n {
                nc *= 10;
                nc += 1;
            }
        }
        _ => {
            nc -= last;
            nc += next_odd_sans_five_digit(last);
        }
    }
    nc
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_next_odd_digit_number() {
        assert_eq!(next_odd_digit_number(0), 1);
        assert_eq!(next_odd_digit_number(1), 3);
        assert_eq!(next_odd_digit_number(19), 31);
        assert_eq!(next_odd_digit_number(3_199_999), 3_311_111);

    }
    #[test]
    fn test_highest_even_digit() {
        assert_eq!(highest_even_digit(0), Some((0, 1)));
        assert_eq!(highest_even_digit(1), None);
        assert_eq!(highest_even_digit(2), Some((2, 1)));
        assert_eq!(highest_even_digit(12), Some((2, 1)));
        assert_eq!(highest_even_digit(121), Some((2, 2)));
        assert_eq!(highest_even_digit(1420), Some((4, 3)));
    }
}
