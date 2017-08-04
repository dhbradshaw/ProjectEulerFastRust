pub fn next_level(a: Vec<u64>, size_increment: i32) -> Vec<u64> {
    let l = a.len();
    match size_increment {
        -1 => {
            let size = l - 1;
            let b: Vec<u64> = (0..size).map(|i| {
                a[i] + a[i + 1]
            }).collect();
            b
        },
        1 => {
            let size = l + 1;
            let b: Vec<u64> = (0..size).map(|i| {
                if i == 0 {
                    return a[0];
                }
                if i == l {
                    return a[l - 1];
                }
                a[i] + a[i - 1]
            }).collect();
            b
        },
        _ => {
            Vec::new()
        }
    }
}

pub fn corner_to_corner(n: u64) -> u64 {
    let mut v = vec![1];
    for i in 0..n {
        v = next_level(v, 1);
    }
    for i in 0..n {
        v = next_level(v, -1);
    }
    v[0]
}

pub fn factorial(n: u64) -> u64 {
    (1..n + 1).fold(1, |p, n| p * n)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_next_level() {
        assert_eq!(next_level(vec![1, 2, 1], 1), vec![1, 3, 3, 1]);
        assert_eq!(next_level(vec![1, 2, 1], -1), vec![3, 3]);
    }
    #[test]
    fn test_corner_to_corner() {
        assert_eq!(corner_to_corner(1), 2);
        assert_eq!(corner_to_corner(2), 6);
    }
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(3), 6);
    }
}
