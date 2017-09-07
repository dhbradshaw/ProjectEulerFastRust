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

pub fn is_triangular(n: u64) -> bool {
    let discriminant = (1 + 8 * n) as f64;
    discriminant.sqrt() % 1f64 == 0f64
}

pub fn hello() {
    let mut t = Triangular::new();
    println!("{} {} {}", t.next().unwrap(), t.next().unwrap(), t.next().unwrap());
    println!("Hello triangle module!")
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_triangular() {
        assert_eq!(is_triangular(1), true);
        assert_eq!(is_triangular(2), false);
        assert_eq!(is_triangular(3), true);
        assert_eq!(is_triangular(4), false);
        assert_eq!(is_triangular(5), false);
        assert_eq!(is_triangular(6), true);
        assert_eq!(is_triangular(7), false);
        assert_eq!(is_triangular(54), false);
        assert_eq!(is_triangular(55), true);
        assert_eq!(is_triangular(56), false);


    }
}
