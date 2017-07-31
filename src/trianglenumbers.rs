pub struct Triangular {
    last_added: u64,
    sum: u64
}

impl Triangular {
    pub fn new() -> Triangular {
        Triangular { last_added: 0, sum: 0 }
    }
}

impl Iterator for Triangular {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.last_added += 1;
        self.sum += self.last_added;
        Some(self.sum)
    }
}

pub fn hello() {
    let mut t = Triangular::new();
    println!("{} {} {}", t.next().unwrap(), t.next().unwrap(), t.next().unwrap());
    println!("Hello triangle module!")
}
