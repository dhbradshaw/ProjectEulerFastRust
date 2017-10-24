pub fn next_level(a: Vec<u64>, size_increment: i32) -> Vec<u64> {
    let l = a.len();
    match size_increment {
        -1 => {
            let size = l - 1;
            let b: Vec<u64> = (0..size).map(|i| a[i] + a[i + 1]).collect();
            b
        }
        1 => {
            let size = l + 1;
            let b: Vec<u64> = (0..size)
                .map(|i| {
                    if i == 0 {
                        return a[0];
                    }
                    if i == l {
                        return a[l - 1];
                    }
                    a[i] + a[i - 1]
                })
                .collect();
            b
        }
        _ => Vec::new(),
    }
}

pub fn corner_to_corner(n: u64) -> u64 {
    let mut v = vec![1];
    for _ in 0..n {
        v = next_level(v, 1);
    }
    for _ in 0..n {
        v = next_level(v, -1);
    }
    v[0]
}

pub fn factorial(n: u64) -> u64 {
    (1..n + 1).fold(1, |p, n| p * n)
}

pub fn corner_to_corner_fast(n: u64) -> u64 {
    // This is just a possibly silly function whose goal is to calculate larger corner to corner_to_corner
    // values without resorting to big numbers.  The strategy is to divide as you multiply instead
    // of calculating (2n)! directly.

    // The first n! just takes out the first half of the terms of (2n)!.
    // 2n! / (n!) * (n!) -> P(n+1..2n+1) / n!

    // The last terms of n!, if doubled, become the evens in (2n)!.  So just do that first.
    let mut numerators: Vec<u64> = ((n + 1)..(2 * n + 1))
        .map(|n| if n % 2 == 0 { 2 } else { n })
        .collect();

    // Now get rid of the rest of the remaining denominator terms.
    let mut multiple = numerators.pop().unwrap();
    let denominators = 1..(n/2 + 1);
    for d in denominators {
        while multiple % d > 0 {
            multiple *= numerators.pop().unwrap();
        }
        multiple /= d;
    }

    // Now get the product of what's left.
    for nu in numerators {
        multiple *= nu;
    }
    multiple
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
    fn test_corner_to_corner_fast() {
        assert_eq!(corner_to_corner_fast(1), 2);
        assert_eq!(corner_to_corner_fast(2), 6);
        assert_eq!(corner_to_corner_fast(20), 137846528820);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(3), 6);
    }
}
