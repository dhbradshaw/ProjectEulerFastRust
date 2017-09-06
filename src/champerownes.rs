pub fn digit_count(n: u32) -> u32 {
    (n as f32).log(10f32) as u32 + 1
}

pub fn digit_value(digit: u32, n: u32) -> u32 {
    let dc = digit_count(n);
    let mut val = 0;
    let mut nc = n;
    for _ in 0..(dc - digit) {
        val = nc % 10;
        nc /= 10;
    }
    val
}

pub fn champerownes_digit(n: u32) -> u32 {
    if n <= 9 {
        return n;
    } else {
        let mut nc = n;
        let mut digits = 1;
        let mut copies = 9;
        while nc > digits * copies {
            nc -= digits * copies;
            digits += 1;
            copies *= 10;
        }
        nc -= 1;
        let which = nc / digits + (copies / 9);
        let digit = nc % digits;
        digit_value(digit, which)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_champerownes_digit() {
        champerownes_digit(9);
        assert_eq!(champerownes_digit(9), 9);
        assert_eq!(champerownes_digit(10), 1);
        assert_eq!(champerownes_digit(11), 0);
        assert_eq!(champerownes_digit(12), 1);
        assert_eq!(champerownes_digit(13), 1);
        assert_eq!(champerownes_digit(14), 1);
        assert_eq!(champerownes_digit(15), 2);

    }

    #[test]
    fn test_digit_value() {
        assert_eq!(digit_value(0, 1), 1);
        assert_eq!(digit_value(0, 2), 2);
        assert_eq!(digit_value(0, 12), 1);
        assert_eq!(digit_value(1, 12), 2);
        assert_eq!(digit_value(1, 321), 2);
    }
}
