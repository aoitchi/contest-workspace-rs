// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        k: Usize1,
    };

    let mut v = vec![];
    for i in 1..10 {
        f(i, &mut v);
    }
    v.sort();
    println!("{}", v[k]);
}

fn f(x: u64, v: &mut Vec<u64>) {
    v.push(x);
    let p = x % 10;
    for i in 0..p {
        f(x * 10 + i, v);
    }
}
