pub fn n_2_string(n: u16) -> String {
    let ones = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    };
    String::from(ones)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_n_2_string() {
        assert_eq!(n_2_string(1), "one");
        assert_eq!(n_2_string(2), "two");
        assert_eq!(n_2_string(3), "three");
        assert_eq!(n_2_string(4), "four");
        assert_eq!(n_2_string(5), "five");
        assert_eq!(n_2_string(6), "six");
        assert_eq!(n_2_string(7), "seven");
        assert_eq!(n_2_string(8), "eight");
        assert_eq!(n_2_string(9), "nine");
        assert_eq!(n_2_string(10), "ten");
    }
}
