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
        wx: [(u64, usize); n],
    };
    let mut cnt = vec![0; 24];
    for &(w, x) in &wx {
        for i in 9..18 {
            cnt[(x + i) % 24] += w;
        }
    }
    println!("{}", cnt.iter().max().unwrap());
}
