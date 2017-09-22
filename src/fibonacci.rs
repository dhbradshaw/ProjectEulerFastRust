pub struct Fibonacci {
    pub last: u64,
    pub curr: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.curr = self.curr + self.last;
        self.last = self.curr - self.last;
        Some(self.curr)
    }
}
