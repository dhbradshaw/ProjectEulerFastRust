use std::cmp::max;
use std::cmp::min;



fn climb(order: usize, sum: usize) -> Vec<(usize, usize)> {
    let largest = order - 1;
    let smallest = 0;

    let mut i = min(largest, sum);
    let mut j = sum - i;
    let mut v = Vec::new();
    loop {
        v.push((i, j));
        if i <= smallest || j >= largest {
            break;
        }
        i = i - 1;
        j = j + 1;
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_climb_first() {
        assert_eq!(climb(1, 0), vec![(0, 0)]);
        assert_eq!(climb(2, 0), vec![(0, 0)]);
        assert_eq!(climb(2, 1), vec![(1, 0), (0, 1)]);
        assert_eq!(climb(2, 2), vec![(1, 1)]);
    }
}
