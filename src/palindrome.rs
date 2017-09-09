pub fn reverse_decimal_digits(n: u32) -> Vec<u32> {
    let mut cp = n;
    let mut digits = Vec::new();
    while cp > 0 {
        digits.push(cp % 10);
        cp = cp / 10;
    }
    digits
}

pub fn is_palindrome(s: &[u32]) -> bool {
    let l = s.len();
    for i in 0..l/2 {
        if s[i] != s[l - 1 - i] {
            return false
        }
    }
    true
}

pub fn binary_palindrome_from_base(base: u64, odd_length: bool) -> u64 {
    let mut left = base;
    let mut right = base;
    if odd_length {
        right /= 2;
        while right > 0 {
            left *= 2;
            left += right % 2;

            right /= 2;
        }
    }
    else {
        while right > 0 {
            left *= 2;
            left += right % 2;

            right /= 2;
        }
    }
    left
}

pub struct BinaryPalindromes {
    pub base: u64,
    pub palindrome: u64,
    pub odd_length: bool,
}

impl BinaryPalindromes {
    pub fn new() -> BinaryPalindromes {
        BinaryPalindromes {base: 0, palindrome: 0, odd_length: false}
    }
}

fn is_power_of_2(n: u64) -> bool {
    n != 0 && n & (n - 1) == 0
}

impl Iterator for BinaryPalindromes {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.base += 1;

        if is_power_of_2(self.base) {
            if self.odd_length {
                self.base /= 2;
                self.odd_length = false;
            } else {
                self.odd_length = true;
            }
        }

        self.palindrome = binary_palindrome_from_base(self.base, self.odd_length);
        Some(self.palindrome)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_binary_palindrome_from_base() {
        assert_eq!(binary_palindrome_from_base(0b1, true), 0b1);
        assert_eq!(binary_palindrome_from_base(0b10, true), 0b101);
        assert_eq!(binary_palindrome_from_base(0b11, true), 0b111);
        assert_eq!(binary_palindrome_from_base(0b100, true), 0b10001);

        assert_eq!(binary_palindrome_from_base(0b1, false), 0b11);
        assert_eq!(binary_palindrome_from_base(0b10, false), 0b1001);
        assert_eq!(binary_palindrome_from_base(0b11, false), 0b1111);
        assert_eq!(binary_palindrome_from_base(0b100, false), 0b100001);
    }
    #[test]
    fn test_binary_palindromes() {
        let mut bps = BinaryPalindromes::new();
        assert_eq!(Some(1), bps.next());
        assert_eq!(Some(3), bps.next());
        assert_eq!(Some(5), bps.next());
    }
}
