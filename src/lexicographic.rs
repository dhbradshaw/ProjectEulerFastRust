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

#[cfg(test)]
mod test {
    use super::*;
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
