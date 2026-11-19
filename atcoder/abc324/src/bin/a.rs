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
        mut a: [u32; n],
    };
    a.sort();
    a.dedup();
    println!("{}", if a.len() == 1 { "Yes" } else { "No" });
}
