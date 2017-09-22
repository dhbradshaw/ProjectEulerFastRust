use std::cmp::max;

pub fn highest_values(upper: &Vec<u32>, lower: &Vec<u32>) -> Vec<u32> {
    let l = upper.len();
    let mut v = Vec::new();
    for i in 0..l {
        let agg = max(lower[i], lower[i + 1]) + upper[i];
        v.push(agg)
    }
    v
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_highest_values() {
        assert_eq!(highest_values(&vec![1], &vec![1, 2]), vec![3]);
        assert_eq!(
            highest_values(&vec![3, 2, 1], &vec![7, 20, 57, 46]),
            vec![23, 59, 58]
        );
    }
}
