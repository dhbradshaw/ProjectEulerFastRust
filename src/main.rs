fn p1(bar: u64) -> u64 {
    println!("If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.");
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

fn main() {
    println!("{}", p1(10));
    println!("{}", p1(1000));
}
