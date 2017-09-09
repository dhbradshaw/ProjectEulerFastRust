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
    for p in primes {
        if *p > ((n as f64).sqrt() as u64) {
            break;
        }
        if n % p == 0 {
            return false
        }
    }
    true
}

pub fn is_prime_no_memo(n: u64) -> bool {
    match n {
        1 => false,
        2 => true,
        n if n % 2 == 0 => false,
        _ => {
            let sqrt = (n as f32).sqrt() as u64;
            let mut test = 3;
            while test <= sqrt {
                if n % test == 0 {
                    return false;
                }
                test += 2;
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
    primes[primes.len()-1]
}

pub fn sieve_10() -> [bool; 10] {
    let mut is_prime = [true; 10];
    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += i;
            }
        }
        i += 1
    }
    is_prime
}

pub fn sieve_16000() -> [bool; 16000] {
    let mut is_prime = [true; 16000];
    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += i;
            }
        }
        i += 1
    }
    is_prime
}

pub fn sieve_1_000_000() -> [bool; 1000000] {
    let mut is_prime = [true; 1000000];
    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += i;
            }
        }
        i += 1
    }
    is_prime
}

pub fn sieve_2_000_000() -> [bool; 2000000] {
    let mut is_prime = [true; 2000000];
    let l = is_prime.len();
    let sqrt = (l as f64).sqrt() as usize;

    let mut i = 2;
    while i < sqrt + 1 {
        if is_prime[i] {
            let mut unprime = i * i;
            while unprime < l {
                is_prime[unprime] = false;
                unprime += i;
            }
        }
        i += 1
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
            break last - first + 1
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
        assert_eq!(sieve_10(), [
            true,
            true,
            true,
            true,
            false,

            true,
            false,
            true,
            false,
            false,
        ]);
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
}
