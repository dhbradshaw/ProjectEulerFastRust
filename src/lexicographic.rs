use std::cmp::min;
// 0123
// 0132
// 0213
// 0231
// 0312
// 0321

// 1023
// 1032
// 1203
// 1230
// 1302
// 1320

// 2013
// 2031
// 2103
// 2130
// 2310
// 2301

// 3012
// 3021
// 3102
// 3120
// 3201
// 3210

fn least_greater(n: u8, slice: &[u8]) -> Option<u8> {
    let greaters: Vec<u8> = slice.iter().filter(|m| {**m > n}).map(|n| *n).collect();
    if greaters.len() > 0 {
        return Some(greaters.iter().fold(255, |a, b| min(a, *b)))
    }
    return None
}

pub fn next(a: &[u8]) -> Option<Vec<u8>> {
    let l = a.len();
    if l < 2 {
        return None
    }
    if l == 2 {
        if a[0] > a[1] {
            return None
        } else {
            return Some(vec![a[1], a[0]])
        }
    } else {
        let head = a[0];
        let tail = &a[1..];
        let next_ = next(tail);
        match next_ {
            Some(next_) => {
                let mut out = vec![head];
                out.extend(next_);
                Some(out)
            }
            None => {
                let lg = least_greater(head, tail);
                match lg {
                    Some(new_head) => {
                        let mut rest: Vec<u8> = a.iter()
                            .filter(|n| **n!=new_head)
                            .map(|n| *n)
                            .collect();
                        rest.sort();
                        let mut out = vec![new_head];
                        out.extend(&rest);
                        Some(out)
                    }
                    None => None
                }
            }
        }
    }
}

pub fn nth_term(n: u32) -> u32 {
    let mut available = [true; 10]; // Track digits 0 through 9.
    let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let mut nc = n - 1;
    let mut i = 9;
    let mut digits = Vec::new();
    while available.iter().filter(|b| **b).count() > 0 {
        let f = factorials[i];
        let digit_index = nc / f;
        nc = nc % f;

        let mut misses = 0;
        for (i, present) in available.iter().enumerate() {
            if !present {
                misses += 1;
            }
            if digit_index + misses == i as u32 {
                digits.push(i);
                break;
            }
        }
        if i > 0 {
            i -= 1;
        }
        available[digits[digits.len() - 1]] = false;
    }
    let mut out = 0usize;
    for d in digits.iter() {
        out *= 10;
        out += *d;
    }
    out as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_nth_term() {
        nth_term(1_000_000);
    }
    #[test]
    fn test_next() {
        assert_eq!(next(&vec![1, 2]), Some(vec![2, 1]));
        assert_eq!(next(&vec![2, 1]), None);
        assert_eq!(next(&vec![0, 1, 2]), Some(vec![0, 2, 1]));
        assert_eq!(next(&vec![0, 2, 1]), Some(vec![1, 0, 2]));
        assert_eq!(next(&vec![2, 1, 0]), None);
    }
    #[test]
    fn test_least_greater() {
        assert_eq!(least_greater(2, &vec![0,1,2,3,4]), Some(3));
        assert_eq!(least_greater(4, &vec![0,1,2,3,4]), None);
    }
}
