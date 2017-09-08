pub fn mod_of_power(value: u64, power: u64, modulus: u64) -> u64 {
    let mut x: u64 = 1;
    for _ in 0..power {
        x = (x * value) % modulus;
    }
    x
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mod_of_power() {
        assert_eq!(mod_of_power(999, 999, 10_000), 8999);

    }
}
