extern crate time;

extern crate eulerrust;

use time::PreciseTime;

#[cfg_attr(rustfmt, rustfmt_skip)]
use eulerrust::problems::{
    p1, p2, p3, p4, p5,
    p6, p7, p8, p9, p10,
    p11, p12, p13, p14, p15,
    p16, p17, p18, p19, p20,
    p21, p22, p23, p24, p25,
    p26, p27, p28, p29, p30,
    p31, p32, p33, p34, p35,
    p36, p37, p38, p39, p40,
    p41, p42, p43, p44, p45,
    p46, p47, p48, p49, p50,
};

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let mut n;
    let start_all = PreciseTime::now();

    let start = PreciseTime::now(); n=p1() as u64; let end = PreciseTime::now(); println!("p1 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p2() as u64; let end = PreciseTime::now(); println!("p2 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p3(); let end = PreciseTime::now(); println!("p3 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p4() as u64; let end = PreciseTime::now(); println!("p4 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p5() as u64; let end = PreciseTime::now(); println!("p5 seconds: {} answer: {:?}", start.to(end), n);

    let start = PreciseTime::now(); n=p6() as u64; let end = PreciseTime::now(); println!("p6 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p7() as u64; let end = PreciseTime::now(); println!("p7 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p8(); let end = PreciseTime::now(); println!("p8 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p9() as u64; let end = PreciseTime::now(); println!("p9 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p10() as u64; let end = PreciseTime::now(); println!("p10 seconds: {} answer: {:?}", start.to(end), n);


    let start = PreciseTime::now(); n=p11() as u64; let end = PreciseTime::now(); println!("p11 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p12() as u64; let end = PreciseTime::now(); println!("p12 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p13() as u64; let end = PreciseTime::now(); println!("p13 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p14() as u64; let end = PreciseTime::now(); println!("p14 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p15() as u64; let end = PreciseTime::now(); println!("p15 seconds: {} answer: {:?}", start.to(end), n);

    let start = PreciseTime::now(); n=p16() as u64; let end = PreciseTime::now(); println!("p16 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p17() as u64; let end = PreciseTime::now(); println!("p17 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p18() as u64; let end = PreciseTime::now(); println!("p18 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p19() as u64; let end = PreciseTime::now(); println!("p19 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p20() as u64; let end = PreciseTime::now(); println!("p20 seconds: {} answer: {:?}", start.to(end), n);


    let start = PreciseTime::now(); n=p21() as u64; let end = PreciseTime::now(); println!("p21 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p22() as u64; let end = PreciseTime::now(); println!("p22 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p23() as u64; let end = PreciseTime::now(); println!("p23 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p24() as u64; let end = PreciseTime::now(); println!("p24 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p25() as u64; let end = PreciseTime::now(); println!("p25 seconds: {} answer: {:?}", start.to(end), n);

    let start = PreciseTime::now(); n=p26() as u64; let end = PreciseTime::now(); println!("p26 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); let n27 = p27(); let end = PreciseTime::now(); println!("p27 seconds: {} answer: {:?}", start.to(end), n27);
    let start = PreciseTime::now(); n=p28() as u64; let end = PreciseTime::now(); println!("p28 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p29() as u64; let end = PreciseTime::now(); println!("p29 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p30() as u64; let end = PreciseTime::now(); println!("p30 seconds: {} answer: {:?}", start.to(end), n);


    let start = PreciseTime::now(); n=p31() as u64; let end = PreciseTime::now(); println!("p31 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p32() as u64; let end = PreciseTime::now(); println!("p32 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p33() as u64; let end = PreciseTime::now(); println!("p33 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p34() as u64; let end = PreciseTime::now(); println!("p34 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p35() as u64; let end = PreciseTime::now(); println!("p35 seconds: {} answer: {:?}", start.to(end), n);

    let start = PreciseTime::now(); n=p36() as u64; let end = PreciseTime::now(); println!("p36 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p37() as u64; let end = PreciseTime::now(); println!("p37 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p38() as u64; let end = PreciseTime::now(); println!("p38 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p39() as u64; let end = PreciseTime::now(); println!("p39 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p40() as u64; let end = PreciseTime::now(); println!("p40 seconds: {} answer: {:?}", start.to(end), n);


    let start = PreciseTime::now(); n=p41() as u64; let end = PreciseTime::now(); println!("p41 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p42() as u64; let end = PreciseTime::now(); println!("p42 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p43() as u64; let end = PreciseTime::now(); println!("p43 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p44() as u64; let end = PreciseTime::now(); println!("p44 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p45() as u64; let end = PreciseTime::now(); println!("p45 seconds: {} answer: {:?}", start.to(end), n);

    let start = PreciseTime::now(); n=p46() as u64; let end = PreciseTime::now(); println!("p46 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p47() as u64; let end = PreciseTime::now(); println!("p47 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p48() as u64; let end = PreciseTime::now(); println!("p48 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p49() as u64; let end = PreciseTime::now(); println!("p49 seconds: {} answer: {:?}", start.to(end), n);
    let start = PreciseTime::now(); n=p50() as u64; let end = PreciseTime::now(); println!("p50 seconds: {} answer: {:?}", start.to(end), n);

    let end_all = PreciseTime::now();
    println!("Total time {}", start_all.to(end_all))
}
