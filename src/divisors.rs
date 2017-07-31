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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn correct_divisors() {
        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(3), vec![1, 3]);
        assert_eq!(divisors(6), vec![1, 2, 3, 6]);
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
        assert_eq!(divisors(15), vec![1, 3, 5, 15]);
        assert_eq!(divisors(21), vec![1, 3, 7, 21]);
        assert_eq!(divisors(28), vec![1, 2, 4, 7, 14, 28]);
    }
}

// 10: 1,2,5,10
// 15: 1,3,5,15
// 21: 1,3,7,21
// 28: 1,2,4,7,14,28
