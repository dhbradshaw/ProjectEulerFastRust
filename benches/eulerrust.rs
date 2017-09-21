#[macro_use]
extern crate bencher;
extern crate eulerrust;

use eulerrust::problems;
use bencher::Bencher;

fn b01(bench: &mut Bencher) {bench.iter(|| {problems::p1()})}
fn b02(bench: &mut Bencher) {bench.iter(|| {problems::p2()})}
fn b03(bench: &mut Bencher) {bench.iter(|| {problems::p3()})}
fn b04(bench: &mut Bencher) {bench.iter(|| {problems::p4()})}
fn b05(bench: &mut Bencher) {bench.iter(|| {problems::p5()})}
fn b06(bench: &mut Bencher) {bench.iter(|| {problems::p6()})}
fn b07(bench: &mut Bencher) {bench.iter(|| {problems::p7()})}
fn b08(bench: &mut Bencher) {bench.iter(|| {problems::p8()})}
fn b09(bench: &mut Bencher) {bench.iter(|| {problems::p9()})}
fn b10(bench: &mut Bencher) {bench.iter(|| {problems::p10()})}

fn b11(bench: &mut Bencher) {bench.iter(|| {problems::p11()})}
fn b12(bench: &mut Bencher) {bench.iter(|| {problems::p12()})}
fn b13(bench: &mut Bencher) {bench.iter(|| {problems::p13()})}
fn b14(bench: &mut Bencher) {bench.iter(|| {problems::p14()})}
fn b15(bench: &mut Bencher) {bench.iter(|| {problems::p15()})}
fn b16(bench: &mut Bencher) {bench.iter(|| {problems::p16()})}
fn b17(bench: &mut Bencher) {bench.iter(|| {problems::p17()})}
fn b18(bench: &mut Bencher) {bench.iter(|| {problems::p18()})}
fn b19(bench: &mut Bencher) {bench.iter(|| {problems::p19()})}
fn b20(bench: &mut Bencher) {bench.iter(|| {problems::p20()})}

fn b21(bench: &mut Bencher) {bench.iter(|| {problems::p21()})}
fn b22(bench: &mut Bencher) {bench.iter(|| {problems::p22()})}
fn b23(bench: &mut Bencher) {bench.iter(|| {problems::p23()})}
fn b24(bench: &mut Bencher) {bench.iter(|| {problems::p24()})}
fn b25(bench: &mut Bencher) {bench.iter(|| {problems::p25()})}
fn b26(bench: &mut Bencher) {bench.iter(|| {problems::p26()})}
fn b27(bench: &mut Bencher) {bench.iter(|| {problems::p27()})}
fn b28(bench: &mut Bencher) {bench.iter(|| {problems::p28()})}
fn b29(bench: &mut Bencher) {bench.iter(|| {problems::p29()})}
fn b30(bench: &mut Bencher) {bench.iter(|| {problems::p30()})}

fn b31(bench: &mut Bencher) {bench.iter(|| {problems::p31()})}
fn b32(bench: &mut Bencher) {bench.iter(|| {problems::p32()})}
fn b33(bench: &mut Bencher) {bench.iter(|| {problems::p33()})}
fn b34(bench: &mut Bencher) {bench.iter(|| {problems::p34()})}
fn b35(bench: &mut Bencher) {bench.iter(|| {problems::p35()})}
fn b36(bench: &mut Bencher) {bench.iter(|| {problems::p36()})}
fn b37(bench: &mut Bencher) {bench.iter(|| {problems::p37()})}
fn b38(bench: &mut Bencher) {bench.iter(|| {problems::p38()})}
fn b39(bench: &mut Bencher) {bench.iter(|| {problems::p39()})}
fn b40(bench: &mut Bencher) {bench.iter(|| {problems::p40()})}

fn b41(bench: &mut Bencher) {bench.iter(|| {problems::p41()})}
fn b42(bench: &mut Bencher) {bench.iter(|| {problems::p42()})}
fn b43(bench: &mut Bencher) {bench.iter(|| {problems::p43()})}
fn b44(bench: &mut Bencher) {bench.iter(|| {problems::p44()})}
fn b45(bench: &mut Bencher) {bench.iter(|| {problems::p45()})}
fn b46(bench: &mut Bencher) {bench.iter(|| {problems::p46()})}
fn b47(bench: &mut Bencher) {bench.iter(|| {problems::p47()})}
fn b48(bench: &mut Bencher) {bench.iter(|| {problems::p48()})}
fn b49(bench: &mut Bencher) {bench.iter(|| {problems::p49()})}
fn b50(bench: &mut Bencher) {bench.iter(|| {problems::p50()})}

benchmark_group!(
    benches,
    // b01,
    // b02,
    // b03,
    // b04,
    // b05,
    //
    // b06,
    // b07,
    // b08,
    // b09,
    // b10,
    //
    //
    // b11,
    // b12,
    // b13,
    // b14,
    // b15,
    //
    // b16,
    // b17,
    // b18,
    // b19,
    // b20,
    //
    //
    // b21,
    // b22,
    // b23,
    // b24,
    // b25,
    //
    // b26,
    // b27,
    // b28,
    // b29,
    // b30,
    //
    //
    // b31,
    // b32,
    b33
    // b34,
    // b35,
    //
    // b36,
    // b37,
    // b38,
    // b39,
    // b40,
    //
    //
    // b41,
    // b42,
    // b43,
    // b44,
    // b45,
    //
    // b46,
    // b47,
    // b48,
    // b49,
    // b50
);
benchmark_main!(benches);
