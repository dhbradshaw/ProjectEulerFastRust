fn spell_under_20(n: u16) -> String {
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

fn spell_under_100(n: u16) -> String {
    let tens = n / 10;
    if tens < 2 {
        return spell_under_20(n)
    }
    let s = match tens {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "i don't know, boss",
    };

    let ones = n % 10;
    let out = match ones {
        0 => format!("{}", s),
        _ => format!("{}-{}", s, spell_under_20(ones))
    };
    out
}

fn spell_under_1001(n: u16) -> String {
    if n == 1000 {
        return String::from("one thousand")
    }

    let hundreds = n / 100;
    let rest = n % 100;
    if hundreds > 0 {
        let hundred_string = spell_under_20(hundreds);
        if rest > 0 {
            let rest_string = spell_under_100(rest);
            return format!("{} hundred and {}", hundred_string, rest_string)
        } else {
            return format!("{} hundred", hundred_string)
        }
    } else {
        return spell_under_100(rest)
    }
}

pub fn letter_count_under_1001(n: u16) -> u32 {
    let chars: Vec<char> = spell_under_1001(n).chars().filter(|c| c.is_alphabetic()).collect();
    chars.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_spell_under_20() {
        assert_eq!(spell_under_20(1), "one");
        assert_eq!(spell_under_20(2), "two");
        assert_eq!(spell_under_20(3), "three");
        assert_eq!(spell_under_20(4), "four");
        assert_eq!(spell_under_20(5), "five");
        assert_eq!(spell_under_20(6), "six");
        assert_eq!(spell_under_20(7), "seven");
        assert_eq!(spell_under_20(8), "eight");
        assert_eq!(spell_under_20(9), "nine");
        assert_eq!(spell_under_20(10), "ten");
    }
    #[test]
    fn test_spell_under_100() {
        assert_eq!(spell_under_100(1), "one");
        assert_eq!(spell_under_100(19), "nineteen");
        assert_eq!(spell_under_100(20), "twenty");
        assert_eq!(spell_under_100(21), "twenty-one");
        assert_eq!(spell_under_100(32), "thirty-two");
        assert_eq!(spell_under_100(43), "forty-three");
        assert_eq!(spell_under_100(54), "fifty-four");
        assert_eq!(spell_under_100(65), "sixty-five");
        assert_eq!(spell_under_100(76), "seventy-six");
        assert_eq!(spell_under_100(87), "eighty-seven");
        assert_eq!(spell_under_100(98), "ninety-eight");
    }
    #[test]
    fn test_spell_under_1001() {
        assert_eq!(spell_under_1001(1), "one");
        assert_eq!(spell_under_1001(19), "nineteen");
        assert_eq!(spell_under_1001(20), "twenty");
        assert_eq!(spell_under_1001(21), "twenty-one");
        assert_eq!(spell_under_1001(32), "thirty-two");
        assert_eq!(spell_under_1001(43), "forty-three");
        assert_eq!(spell_under_1001(54), "fifty-four");
        assert_eq!(spell_under_1001(65), "sixty-five");
        assert_eq!(spell_under_1001(76), "seventy-six");
        assert_eq!(spell_under_1001(87), "eighty-seven");
        assert_eq!(spell_under_1001(98), "ninety-eight");
        assert_eq!(spell_under_1001(342), "three hundred and forty-two");
        assert_eq!(spell_under_1001(1000), "one thousand");
    }
    #[test]
    fn test_letter_count_under_1001() {
        assert_eq!(letter_count_under_1001(342), 23);
        assert_eq!(letter_count_under_1001(115), 20);
        assert_eq!(letter_count_under_1001(1000), 11);

    }
}
