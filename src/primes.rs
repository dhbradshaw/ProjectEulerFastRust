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


#[cfg(test)]
mod test {
    use super::*;
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
}
