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
}
