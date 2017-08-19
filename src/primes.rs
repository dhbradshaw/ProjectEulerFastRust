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
