pub struct Collatz {
    n: u64
}

impl Collatz {
    pub fn new(n: u64) -> Collatz {
        Collatz { n: n }
    }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n % 2 == 0 {
            self.n = self.n / 2;
            return Some(self.n)
        }
        if self.n == 1 {
            return None
        }
        self.n = self.n * 3 + 1;
        Some(self.n)
    }
}

pub fn count_collatz(n: usize, lengths: &mut [usize; 500_000]) -> usize {
    if n == 1 {
        return 1;
    }
    if n < 500_000 && lengths[n] > 0 {
        return lengths[n];
    }
    let val = 1 + match n % 2 {
        0 => count_collatz(n / 2, lengths),
        _ => count_collatz(n * 3 + 1, lengths)
    };
    if n < 500_000 {
        lengths[n] = val;
    }
    val
}

pub fn longest_collatz_memo(highest: usize) -> usize {
    let mut max_length = 0;
    let mut lengths: [usize; 500_000] = [0; 500_000];
    let mut cause = 0;
    for i in 1..highest + 1 {
        let length = count_collatz(i, &mut lengths);
        if length > max_length {
            cause = i;
            max_length = length;
        }
    }
    cause
}

pub fn longest_collatz(highest: usize) -> usize {
    let mut max_length = 0;
    let mut cause = 0;
    for i in 1..highest + 1 {
        let mut length = 1;
        let mut n = i;
        loop {
            if n == 1 {
                break;
            }
            n = match n % 2 {
                0 => n / 2,
                _ => n * 3 + 1
            };
            length += 1;
        }
        if length > max_length {
            max_length = length;
            cause = i;
        }
    }
    cause
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn collatz_13() {
        let mut collatz = Collatz::new(13);
        assert_eq!(collatz.next(), Some(40));
        assert_eq!(collatz.next(), Some(20));
        assert_eq!(collatz.next(), Some(10));
        assert_eq!(collatz.next(), Some(5));
        assert_eq!(collatz.next(), Some(16));
        assert_eq!(collatz.next(), Some(8));
        assert_eq!(collatz.next(), Some(4));
        assert_eq!(collatz.next(), Some(2));
        assert_eq!(collatz.next(), Some(1));
        assert_eq!(collatz.next(), None);
    }
    #[test]
    fn test_longest_collatz() {
        assert_eq!(longest_collatz(1), 1);
        assert_eq!(longest_collatz(2), 2);
        assert_eq!(longest_collatz(3), 3)

    }
    #[test]
    fn test_longest_collatz_memo() {
        assert_eq!(longest_collatz_memo(1), 1);
    }
}
