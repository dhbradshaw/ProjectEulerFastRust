extern crate chrono;
extern crate num;
extern crate time;
extern crate eulerrust;
use num::FromPrimitive;
use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use time::PreciseTime;
use chrono::{Datelike, NaiveDate, Weekday};
use eulerrust::divisors::is_amicable;


#[allow(dead_code)]
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

#[allow(dead_code)]
fn p1_iterate(bar: u64) -> u64 {
    (1..bar).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[test]
fn test_p1_iterate() {
    assert!(p1_iterate(10) == 23)
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn p5() -> u64 {
    // What is the smallest positive number that is evenly divisible
    // by all of the numbers from 1 to 20?
    1 * 2 * 3 * 2 * 5 * 7 * 2 * 3 * 11 * 13 * 2 * 17 * 19
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn p7() -> u64 {
    let start = PreciseTime::now();
    let out = nth_prime(10001);
    let end = PreciseTime::now();
    println!("p7 time: {} seconds", start.to(end));
    out
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn p13() -> String {
    // Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
    let s="37107287533902102798797998220837590246510135740250
        46376937677490009712648124896970078050417018260538
        74324986199524741059474233309513058123726617309629
        91942213363574161572522430563301811072406154908250
        23067588207539346171171980310421047513778063246676
        89261670696623633820136378418383684178734361726757
        28112879812849979408065481931592621691275889832738
        44274228917432520321923589422876796487670272189318
        47451445736001306439091167216856844588711603153276
        70386486105843025439939619828917593665686757934951
        62176457141856560629502157223196586755079324193331
        64906352462741904929101432445813822663347944758178
        92575867718337217661963751590579239728245598838407
        58203565325359399008402633568948830189458628227828
        80181199384826282014278194139940567587151170094390
        35398664372827112653829987240784473053190104293586
        86515506006295864861532075273371959191420517255829
        71693888707715466499115593487603532921714970056938
        54370070576826684624621495650076471787294438377604
        53282654108756828443191190634694037855217779295145
        36123272525000296071075082563815656710885258350721
        45876576172410976447339110607218265236877223636045
        17423706905851860660448207621209813287860733969412
        81142660418086830619328460811191061556940512689692
        51934325451728388641918047049293215058642563049483
        62467221648435076201727918039944693004732956340691
        15732444386908125794514089057706229429197107928209
        55037687525678773091862540744969844508330393682126
        18336384825330154686196124348767681297534375946515
        80386287592878490201521685554828717201219257766954
        78182833757993103614740356856449095527097864797581
        16726320100436897842553539920931837441497806860984
        48403098129077791799088218795327364475675590848030
        87086987551392711854517078544161852424320693150332
        59959406895756536782107074926966537676326235447210
        69793950679652694742597709739166693763042633987085
        41052684708299085211399427365734116182760315001271
        65378607361501080857009149939512557028198746004375
        35829035317434717326932123578154982629742552737307
        94953759765105305946966067683156574377167401875275
        88902802571733229619176668713819931811048770190271
        25267680276078003013678680992525463401061632866526
        36270218540497705585629946580636237993140746255962
        24074486908231174977792365466257246923322810917141
        91430288197103288597806669760892938638285025333403
        34413065578016127815921815005561868836468420090470
        23053081172816430487623791969842487255036638784583
        11487696932154902810424020138335124462181441773470
        63783299490636259666498587618221225225512486764533
        67720186971698544312419572409913959008952310058822
        95548255300263520781532296796249481641953868218774
        76085327132285723110424803456124867697064507995236
        37774242535411291684276865538926205024910326572967
        23701913275725675285653248258265463092207058596522
        29798860272258331913126375147341994889534765745501
        18495701454879288984856827726077713721403798879715
        38298203783031473527721580348144513491373226651381
        34829543829199918180278916522431027392251122869539
        40957953066405232632538044100059654939159879593635
        29746152185502371307642255121183693803580388584903
        41698116222072977186158236678424689157993532961922
        62467957194401269043877107275048102390895523597457
        23189706772547915061505504953922979530901129967519
        86188088225875314529584099251203829009407770775672
        11306739708304724483816533873502340845647058077308
        82959174767140363198008187129011875491310547126581
        97623331044818386269515456334926366572897563400500
        42846280183517070527831839425882145521227251250327
        55121603546981200581762165212827652751691296897789
        32238195734329339946437501907836945765883352399886
        75506164965184775180738168837861091527357929701337
        62177842752192623401942399639168044983993173312731
        32924185707147349566916674687634660915035914677504
        99518671430235219628894890102423325116913619626622
        73267460800591547471830798392868535206946944540724
        76841822524674417161514036427982273348055556214818
        97142617910342598647204516893989422179826088076852
        87783646182799346313767754307809363333018982642090
        10848802521674670883215120185883543223812876952786
        71329612474782464538636993009049310363619763878039
        62184073572399794223406235393808339651327408011116
        66627891981488087797941876876144230030984490851411
        60661826293682836764744779239180335110989069790714
        85786944089552990653640447425576083659976645795096
        66024396409905389607120198219976047599490197230297
        64913982680032973156037120041377903785566085089252
        16730939319872750275468906903707539413042652315011
        94809377245048795150954100921645863754710598436791
        78639167021187492431995700641917969777599028300699
        15368713711936614952811305876380278410754449733078
        40789923115535562561142322423255033685442488917353
        44889911501440648020369068063960672322193204149535
        41503128880339536053299340368006977710650566631954
        81234880673210146739058568557934581403627822703280
        82616570773948327592232845941706525094512325230608
        22918802058777319719839450180888072429661980811197
        77158542502016545090413245809786882778948721859617
        72107838435069186155435662884062257473692284509516
        20849603980134001723930671666823555245252804609722
        53503534226472524250874054075591789781264330331690";
    let total: u64 = s.lines().map(|line| {
        let l: String = line.chars().filter(|a| a.is_digit(10)).take(12).collect();
        let n = l.parse::<u64>().unwrap();
        n
    }).sum();
    let first10: String = total.to_string().chars().take(10).collect();
    first10
}

#[allow(dead_code)]
fn p14() -> usize {
    eulerrust::collatz::longest_collatz(999_999)
}

#[allow(dead_code)]
fn p15() -> u64 {
    eulerrust::lattice::corner_to_corner(20)
}

#[allow(dead_code)]
fn p15_factorial() -> u64 {
    let f_20 = eulerrust::lattice::factorial(20);
    eulerrust::lattice::factorial(40)/(f_20 * f_20)
}

#[allow(dead_code)]
fn p16() -> u32 {
    num::pow(num::BigUint::new(vec![2]), 1000)
        .to_str_radix(10)
        .chars()
        .map(|c| {c.to_digit(10).unwrap()})
        .sum()
}

#[allow(dead_code)]
fn p17() -> u32 {
    let mut total = 0;
    for i in 1..1001 {
        total += eulerrust::numberletters::letter_count_under_1001(i);
    }
    total
}

#[allow(dead_code)]
fn p18() -> u32 {
    let triangle: Vec<Vec<u32>> = vec![
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
        vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![99, 65, 04, 28, 06, 16, 70, 92,],
        vec![88, 02, 77, 73, 07, 63, 67,],
        vec![19, 01, 23, 75, 03, 34],
        vec![20, 04, 82, 47, 65],
        vec![18, 35, 87, 10],
        vec![17, 47, 82],
        vec![95, 64],
        vec![75],
    ];
    let mut agg = (&triangle[0]).to_vec();
    for i in 0..(triangle.len() - 1) {
        agg = eulerrust::maxpathsum::highest_values(&triangle[i+1], &agg);
    }
    agg[0]
}

#[allow(dead_code)]
fn p19() -> u32 {
    let mut count = 0;
    for year in 1901..2001 {
        for month in 1..13 {
            if NaiveDate::from_ymd(year, month, 1).weekday() == Weekday::Sun {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn p20() -> u32 {
    let mut agg = num::BigUint::from(1 as u32);
    for i in 1..101 {
        agg = agg * num::BigUint::from(i as u32)
    }
    agg.to_str_radix(10)
        .chars()
        .map(|a| a.to_digit(10).unwrap())
        .sum()
}

#[allow(dead_code)]
fn p21() -> u32 {
    (1..10000).filter(|n| is_amicable(*n as u64)).sum()
}

#[allow(dead_code)]
fn p22() -> u32 {
    // Read in the file
    let mut file = File::open("p022_names.txt").unwrap();
    let mut names = String::new();
    file.read_to_string(&mut names).unwrap();

    // Convert names to vectors of integers and sort
    let zero = 'A' as u32 - 1;
    let mut names: Vec<Vec<_>> = names.split(",").map(
        |s| {
            s.chars()
            .filter(|c| {c.is_alphabetic()})
            .map(|c| {c as u32 - zero})
            .collect()
        }
    ).collect();
    names.sort();

    // Total scores
    let mut sum = 0;
    for (i, name) in names.iter().enumerate() {
        let place: u32 = i as u32 + 1;
        let score: u32 = name.iter().sum();
        sum += place * score;
    }
    sum
}

#[allow(dead_code)]
fn p23() -> u32 {
    // Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
    let abundants = eulerrust::divisors::abundants_below(28124);
    let abundant_set: HashSet<u32> = HashSet::from_iter(abundants.iter().cloned());
    let mut total = 0;
    for n in 1..28124 {
        let too_high = n / 2 + 1;
        let mut is_abundant_sum = false;
        for abundant in &abundants {
            if *abundant >=  too_high {
                break;
            }
            let partner = n - *abundant;
            if abundant_set.contains(&partner) {
                is_abundant_sum = true;
                break;
            }
        }
        if !is_abundant_sum {
            total += n;
        }
    }
    total
}

#[allow(dead_code)]
fn p24() -> Vec<u8> {
    let mut count = 1;
    let mut v = vec![0,1,2,3,4,5,6,7,8,9];
    loop {
        v = eulerrust::lexicographic::next(&v).unwrap();
        count += 1;
        if count == 1000000 {
            break v
        }
    }
}

#[allow(dead_code)]
fn p25() -> u32 {
    let mut n = 2;

    let ten = num::BigUint::from_u32(10).unwrap();
    let bound = num::pow(ten, 999);
    let mut current = num::BigUint::new(vec![1]);
    let mut last = num::BigUint::new(vec![1]);

    while current < bound {
        let sum = &current + last;
        last = current;
        current = sum;
        n += 1;
    }
    n
}

fn main() {
    // println!("{}", p1(10));
    // println!("{}", p1_iterate(10));
    // println!("{}", p1(1000));``
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
    // p12();
    // let n = p13();
    // let n = p14();
    // let mut n = p15();
    // n = p15_factorial();
    // let n = p16();
    // let n = p17();
    // let n = p18();
    // let n = p19();
    // let n = p20();
    // let n = p21();
    // let n = p22();
    // let n = p23();
    // let n = p24();
    let start = PreciseTime::now();
    let n = p25();
    let end = PreciseTime::now();
    println!("seconds: {} answer: {:?}", start.to(end), n);
}
