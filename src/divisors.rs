pub fn divisors(n: u64) -> Vec<u64> {
    let limit = (n as f64).sqrt() as u64;
    let mut v0 = Vec::new();
    let mut v1 = Vec::new();
    let mut test = 1;
    while test <= limit {
        if n % test == 0 {
            v0.push(test);
            let partner = n / test;
            if partner != test {
                v1.push(partner);
            }
        }
        test += 1;
    }
    for partner in v1.iter().rev() {
        v0.push(*partner);
    }
    v0
}

pub fn proper_divisors(n: u64) -> Vec<u64> {
    let mut v = divisors(n);
    v.pop();
    v
}

pub fn proper_divisor_sum(n: u64) -> u64 {
    let s: u64 = divisors(n).iter().sum();
    s - n
}

pub fn is_amicable(n: u64) -> bool {
    let m = proper_divisor_sum(n);
    n != m && n == proper_divisor_sum(m)
}

pub fn is_abundant(n: u64) -> bool {
    proper_divisor_sum(n) > n
}

pub fn abundants_below(n: u32) -> Vec<u32> {
    let mut abundants = Vec::new();
    for i in 1..n {
        if is_abundant(i as u64) {
            abundants.push(i as u32);
        }
    }
    abundants
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn correct_divisors() {
        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(3), vec![1, 3]);
        assert_eq!(divisors(6), vec![1, 2, 3, 6]);
        assert_eq!(divisors(9), vec![1, 3, 9]);
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
        assert_eq!(divisors(15), vec![1, 3, 5, 15]);
        assert_eq!(divisors(21), vec![1, 3, 7, 21]);
        assert_eq!(divisors(27), vec![1, 3, 9, 27]);
        assert_eq!(divisors(28), vec![1, 2, 4, 7, 14, 28]);
        assert_eq!(divisors(32), vec![1, 2, 4, 8, 16, 32]);
    }
    #[test]
    fn test_proper_divisor_sum() {
        assert_eq!(proper_divisor_sum(220), 284);
        assert_eq!(proper_divisor_sum(284), 220);
    }
    #[test]
    fn test_is_amicable() {
        assert_eq!(is_amicable(220), true);
        assert_eq!(is_amicable(284), true);
        assert_eq!(is_amicable(9), false);
    }
    #[test]
    fn test_is_abundant() {
        let v: Vec<u64> = (1..13)
            .map(|n| {n as u64})
            .filter(|n| {is_abundant(*n)})
            .collect();
        assert_eq!(v, vec![12 as u64]);
    }
    #[test]
    fn test_abundants_below() {
        let mut v = Vec::new();
        v.push(12 as u32);
        assert_eq!(abundants_below(13), v)
    }
}
