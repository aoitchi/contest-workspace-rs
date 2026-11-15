// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        n: usize,
    };
    for i in 0..=n {
        let x = (1..=9).find(|j| n % j == 0 && i % (n / j) == 0);
        if let Some(x) = x {
            print!("{x}");
        } else {
            print!("-");
        }
    }
    println!();
}
