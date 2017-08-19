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
