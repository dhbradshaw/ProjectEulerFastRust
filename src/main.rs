extern crate time;
extern crate eulerrust;
use std::cmp::max;
use std::cmp::min;
use time::PreciseTime;

fn p1(bar: u64) -> u64 {
    /// If we list all the natural numbers below 10 that are multiples of 3 or 5,
    /// we get 3, 5, 6 and 9. The sum of these multiples is 23.
    /// Find the sum of all the multiples of 3 or 5 below 1000.");
    let mut n = 1;
    let mut agg = 0;
    loop {
        if n == bar {
            break;
        }
        if n % 3 == 0 || n % 5 == 0 {
            agg += n;
        }
        n += 1;
    }
    agg
}

#[test]
fn test_p1() {
    assert!(p1(10) == 23)
}

fn p1_iterate(bar: u64) -> u64 {
    (1..bar).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[test]
fn test_p1_iterate() {
    assert!(p1_iterate(10) == 23)
}

fn p2() -> u64 {
    ///  Each new term in the Fibonacci sequence is generated by adding the previous two terms.
    ///  By starting with 1 and 2, the first 10 terms will be:
    ///      1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
    ///  By considering the terms in the Fibonacci sequence whose values do not exceed
    ///  four million, find the sum of the even-valued terms.
    let mut t1: u64 = 1;
    let mut t2: u64 = 2;
    let mut total: u64 = 0;
    while t2 <= 4000000 {
        if t2 % 2 == 0 {
            total += t2;
        }
        t2 = t1 + t2;
        t1 = t2 - t1;
    }
    total
}

struct Fibonacci {
	last: u64,
	curr: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.curr = self.curr + self.last;
        self.last = self.curr - self.last;
        Some(self.curr)
    }
}

fn p2_iterative() -> u64 {
    let f = Fibonacci{last: 0, curr:1};
    let mut agg: u64 = 0;
    for n in f {
        if n % 2 == 0 {
            agg += n;
        }
        if n >= 4000000 {
            break;
        }
    }
    agg
}

fn p3(n: u64) -> u64 {
    /// The prime factors of 13195 are 5, 7, 13 and 29.
    /// What is the largest prime factor of the number 600851475143 ?
    let mut upper_limit = (n as f64).sqrt() as u64 + 1;
    let mut target = n;
    let mut factor = 2;
    while factor <= upper_limit {
        while target % factor == 0 {
            target = target / factor;
        }
        factor += 1;
        upper_limit = (target as f64).sqrt() as u64 + 1;
    }
    max(target, factor)
}

fn reverse_decimal_digits(n: u32) -> Vec<u32> {
    let mut cp = n;
    let mut digits = Vec::new();
    while cp > 0 {
        digits.push(cp % 10);
        cp = cp / 10;
    }
    digits
}

fn is_palindrome(s: &[u32]) -> bool {
    let l = s.len();
    for i in 0..l/2 {
        if s[i] != s[l - 1 - i] {
            return false
        }
    }
    true
}

fn p4() -> u32 {
    /// A palindromic number reads the same both ways.
    /// The largest palindrome made from the product of two 2-digit numbers is
    /// 9009 = 91 × 99.
    /// Find the largest palindrome made from the product of two 3-digit numbers.
    let mut largest_palindrome = 0;
    let mut i = 999;
    while i > 99 {
        let mut j = i;
        while j > 99 {
            let multiple = i * j;
            if multiple > largest_palindrome {
                let digits = reverse_decimal_digits(multiple);
                if is_palindrome(&digits) {
                    largest_palindrome = multiple
                }
            } else {
                break;
            }
            j -= 1;
        }
        i -= 1;
    }
    largest_palindrome
}

fn p5() -> u64 {
    // What is the smallest positive number that is evenly divisible
    // by all of the numbers from 1 to 20?
    1 * 2 * 3 * 2 * 5 * 7 * 2 * 3 * 11 * 13 * 2 * 17 * 19
}

fn p6() -> u64 {
    let mut sum_of_squares = 0;
    let mut sum = 0;
    for i in 1..101 {
        sum += i;
        sum_of_squares += i * i;
    }
    sum * sum - sum_of_squares
}

fn is_prime(n: u64, primes: &[u64]) -> bool {
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

fn nth_prime(n: usize) -> u64 {
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

fn p7() -> u64 {
    let start = PreciseTime::now();
    let out = nth_prime(10001);
    let end = PreciseTime::now();
    println!("p7 time: {} seconds", start.to(end));
    out
}

fn p8(n: usize) -> u64 {
    let s = "73167176531330624919225119674426574742355349194934
    96983520312774506326239578318016984801869478851843
    85861560789112949495459501737958331952853208805511
    12540698747158523863050715693290963295227443043557
    66896648950445244523161731856403098711121722383113
    62229893423380308135336276614282806444486645238749
    30358907296290491560440772390713810515859307960866
    70172427121883998797908792274921901699720888093776
    65727333001053367881220235421809751254540594752243
    52584907711670556013604839586446706324415722155397
    53697817977846174064955149290862569321978468622482
    83972241375657056057490261407972968652414535100474
    82166370484403199890008895243450658541227588666881
    16427171479924442928230863465674813919123162824586
    17866458359124566529476545682848912883142607690042
    24219022671055626321111109370544217506941658960408
    07198403850962455444362981230987879927244284909188
    84580156166097919133875499200524063689912560717606
    05886116467109405077541002256983155200055935729725
    71636269561882670428252483600823257530420752963450";
    let v = s.chars().filter_map(|a| a.to_digit(10)).collect::<Vec<u32>>();
    let l = v.len();

    let mut largest = 0;
    let mut i = 0;
    while (i + n) < l {
        let candidate = (i..(i + n)).fold(1, |multiple, index| {multiple * (v[index]) as u64});
        largest = max(largest, candidate);
        i += 1;
    }
    largest
}

fn p9() {
    for a in 1..300 {
        let b_numerator = 500000 - 1000 * a;
        let b_denominator = 1000 - a;
        if b_numerator % b_denominator == 0 {
            let b = b_numerator / b_denominator;
            let c = 1000 - a - b;
            println!("{}", a * b * c);
            break;
        }
    }
}

fn primes_below(n: u64) -> Vec<u64> {
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

fn p10() -> u64 {
    // Find the sum of all the primes below two million.
    primes_below(2000000).iter().sum()
}

fn greatest_multiple(v: Vec<u64>, n: usize) -> u64 {
    let l = v.len();
    let mut start = 0;
    let mut largest = 0;
    while (start + n) <= l {
        let multiple = &v[start..start + n].iter()
            .fold(1, |multiple, e| {multiple * (*e) as u64});
        largest = max(largest, *multiple);
        start += 1;
    }
    largest
}

fn p11 () {
    // What is the greatest product of four adjacent numbers in the same direction
    // (up, down, left, right, or diagonally) in the 20×20 grid?
    let s = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
        49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
        81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
        52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
        22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
        24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
        32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
        67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
        24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
        21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
        78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
        16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
        86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
        19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
        04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
        88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
        04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
        20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
        20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
        01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";
    let matrix: Vec<Vec<u64>> = s.lines().map(
        |line| {line.split_whitespace().map(
            |digits| {digits.parse::<u64>().unwrap()}
        ).collect()}
    ).collect();

    let order = matrix.len();
    let am = eulerrust::AbstractMatrix::new(order);

    let mut winner = 0;
    let mut vecs = am.rows();
    vecs.extend(am.columns());
    vecs.extend(am.climbs());
    vecs.extend(am.descends());
    for row in vecs {
        let nums: Vec<u64> = row.iter().map(|p| {
            let &(i, j) = p;
            matrix[i][j]
        }).collect();
        winner = max(greatest_multiple(nums, 4), winner);
    }
    println!("{}", winner);
}

fn p12() {
    // What is the value of the first triangle number to have over five hundred divisors?
    let mut t = eulerrust::trianglenumbers::Triangular::new();
    loop {
        let n = t.next().unwrap();
        if eulerrust::divisors::divisors(n).len() > 500 {
            println!("{}", n);
            break;
        }
    }
}

fn main() {
    // println!("{}", p1(10));
    // println!("{}", p1_iterate(10));
    // println!("{}", p1(1000));
    // println!("{}", p1_iterate(1000));
    // println!("{}", p2());
    // println!("{}", p2_iterative());
    // println!("{}", p3(600851475143));
    // let start = PreciseTime::now();
    // println!("{:?}", p4());
    // let end = PreciseTime::now();
    // println!("{} seconds", start.to(end));
    // println!("{}", p5());
    // println!("{}", p6());
    // println!("{}", p7());
    // println!("{:?}", p8(13)); // prints [1, 2, 3, 4, 5, 6]
    // p9();
    // println!("{}", p10());
    // p11();
    let start = PreciseTime::now();
    p12();
    let end = PreciseTime::now();
    println!("{} seconds", start.to(end));
}
