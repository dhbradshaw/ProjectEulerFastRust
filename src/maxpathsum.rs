pub fn one() -> u64 {
    1
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn dummy_test() {
        assert_eq!(one(), 1);

    }
}
