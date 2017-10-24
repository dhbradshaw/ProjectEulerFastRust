use std::collections::HashSet;

pub fn primes_below(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    primes.push(2);
    let mut candidate = 1;
    loop {
        candidate += 2;
        if candidate >= n {
            break;
        }
        if is_prime(candidate, &primes) {
            primes.push(candidate);
        }
    }
    primes
}

pub fn is_prime(n: u64, primes: &[u64]) -> bool {
    let limit = (n as f64).sqrt() as u64;
    for p in primes {
        if *p > limit {
            break;
        }
        if n % p == 0 {
            return false;
        }
    }
    true
}

pub fn is_prime_no_memo(n: u64) -> bool {
    match n {
        0 => false,
        1 => false,
        2 => true,
        3 => true,
        n if n % 2 == 0 => false,
        n if n % 3 == 0 => false,
        _ => {
            let sqrt = (n as f32).sqrt() as u64;
            let mut base = 6;
            while base <= sqrt {
                if n % (base - 1) == 0 {
                    return false;
                }
                if n % (base + 1) == 0 {
                    return false;
                }
                base += 6;
            }
            true
        }
    }
}

pub fn nth_prime(n: usize) -> u64 {
    let mut primes = Vec::new();
    primes.push(2);
    let mut test = 1;
    while primes.len() < n {
        test += 2;
        if is_prime(test, &primes) {
            primes.push(test);
        }
    }
    primes[primes.len() - 1]
}

pub fn sieve_10() -> [bool; 10] {
    let mut is_prime = [true; 10];
    is_prime[0] = false;
    is_prime[1] = false;

    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    let mut unprime = i * i;
    while unprime < l {
        is_prime[unprime] = false;
        unprime += i;
    }
    i = 3;

    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += 2 * i;
            }
        }
        i += 2
    }
    is_prime
}

pub fn sieve_16000() -> [bool; 16000] {
    let mut is_prime = [true; 16000];
    is_prime[0] = false;
    is_prime[1] = false;

    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    let mut unprime = i * i;
    while unprime < l {
        is_prime[unprime] = false;
        unprime += i;
    }

    i = 3;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += 2 * i;
            }
        }
        i += 2;
    }
    is_prime
}

pub fn sieve_150_000() -> [bool; 150_000] {
    let mut is_prime = [true; 150_000];
    is_prime[0] = false;
    is_prime[1] = false;

    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    let mut unprime = i * i;
    while unprime < l {
        is_prime[unprime] = false;
        unprime += i;
    }

    i = 3;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += 2 * i;
            }
        }
        i += 2;
    }
    is_prime
}

pub fn sieve_1_000_000() -> [bool; 1_000_000] {
    let mut is_prime = [true; 1_000_000];
    is_prime[0] = false;
    is_prime[1] = false;

    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    let mut unprime = i * i;
    while unprime < l {
        is_prime[unprime] = false;
        unprime += i;
    }

    i = 3;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += 2 * i;
            }
        }
        i += 2;
    }
    is_prime
}

pub fn sieve_2_000_000() -> [bool; 2000000] {
    let mut is_prime = [true; 2000000];
    is_prime[0] = false;
    is_prime[1] = false;

    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    let mut unprime = i * i;
    while unprime < l {
        is_prime[unprime] = false;
        unprime += i;
    }

    i = 3;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += 2 * i;
            }
        }
        i += 2;
    }
    is_prime
}

pub fn distinct_prime_factors(n: u64, primes: &Vec<u64>) -> HashSet<u64> {
    let mut nc = n;
    let mut factors = HashSet::new();
    for p in primes.iter() {
        while nc % *p == 0 {
            nc /= *p;
            factors.insert(*p);
        }
        if *p * *p > nc {
            if nc != 1 {
                factors.insert(nc);
            }
            break;
        }
    }
    factors
}

pub fn distinct_prime_factor_count(n: u64, primes: &Vec<u64>) -> u64 {
    let mut nc = n;
    let mut count = 0;
    for p in primes.iter() {
        let mut factor = false;
        while nc % *p == 0 {
            nc /= *p;
            factor = true;
        }
        if factor {
            count += 1;
        }
        if *p * *p > nc {
            if nc != 1 {
                count += 1;
            }
            break;
        }
    }
    count
}

pub fn nonself_prime_factor_counts_200_000() -> [u8; 200_000] {
    let mut counts = [0u8; 200_000];
    let length = counts.len();
    let mut n = 2usize;
    let mut index = n * 2;
    while index < length {
        counts[index] += 1;
        index += n;
    }
    n = 3;
    while n < length / 2 {
        if counts[n] == 0 {
            index = 2 * n;
            while index < length {
                counts[index] += 1;
                index += n;
            }
        }
        n += 2;
    }
    counts
}

pub fn most_consecutive_primes(n: u64, primes: &[u64]) -> usize {
    let mut first = 0usize;
    let mut last = 1usize;
    let mut s = primes[first] + primes[last];
    loop {
        if primes[last] > n || primes[first] > n {
            break 0;
        }
        if s < n {
            last += 1;
            s += primes[last];
        }
        if s == n {
            break last - first + 1;
        }
        if s > n {
            s -= primes[first];
            first += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_most_consecutive_primes() {
        let primes = primes_below(100);
        assert_eq!(most_consecutive_primes(3, &primes), 1);
        assert_eq!(most_consecutive_primes(41, &primes), 6);
        assert_eq!(most_consecutive_primes(953, &primes), 21);
    }
    #[test]
    fn test_sieve() {
        assert_eq!(
            sieve_10(),
            [
                false,
                false,
                true,
                true,
                false,

                true,
                false,
                true,
                false,
                false,
            ]
        );
    }
    #[test]
    fn test_distinct_prime_factors() {
        let primes = primes_below(100);
        let hs: HashSet<u64> = [2].iter().cloned().collect();
        assert_eq!(distinct_prime_factors(2, &primes), hs);

        let hs: HashSet<u64> = [2, 3].iter().cloned().collect();
        assert_eq!(distinct_prime_factors(6, &primes), hs);

        let hs: HashSet<u64> = [2, 5, 13].iter().cloned().collect();
        assert_eq!(distinct_prime_factors(1300, &primes), hs);
    }
    #[test]
    fn test_nonself_prime_factor_counts_200_000() {
        let counts = nonself_prime_factor_counts_200_000();
        assert_eq!(counts[0], 0);
        assert_eq!(counts[1], 0);
        assert_eq!(counts[2], 0);
        assert_eq!(counts[3], 0);
        assert_eq!(counts[4], 1);
        assert_eq!(counts[5], 0);
        assert_eq!(counts[6], 2);
        assert_eq!(counts[7], 0);
        assert_eq!(counts[8], 1);
        assert_eq!(counts[9], 1);
        assert_eq!(counts[10], 2);
        assert_eq!(counts[11], 0);
        assert_eq!(counts[24], 2);
        assert_eq!(counts[30], 3);
    }
    #[test]
    fn test_is_prime_no_memo() {
        assert_eq!(is_prime_no_memo(0), false);
        assert_eq!(is_prime_no_memo(1), false);
        assert_eq!(is_prime_no_memo(2), true);
        assert_eq!(is_prime_no_memo(3), true);
        assert_eq!(is_prime_no_memo(4), false);
        assert_eq!(is_prime_no_memo(5), true);
        assert_eq!(is_prime_no_memo(6), false);
        assert_eq!(is_prime_no_memo(7), true);
        assert_eq!(is_prime_no_memo(8), false);
        assert_eq!(is_prime_no_memo(9), false);
        assert_eq!(is_prime_no_memo(10), false);
        assert_eq!(is_prime_no_memo(11), true);
        assert_eq!(is_prime_no_memo(12), false);
        assert_eq!(is_prime_no_memo(13), true);
        assert_eq!(is_prime_no_memo(14), false);
        assert_eq!(is_prime_no_memo(15), false);
        assert_eq!(is_prime_no_memo(16), false);
        assert_eq!(is_prime_no_memo(17), true);
        assert_eq!(is_prime_no_memo(18), false);
        assert_eq!(is_prime_no_memo(19), true);
        assert_eq!(is_prime_no_memo(20), false);
    }
}
