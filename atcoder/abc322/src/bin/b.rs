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
        m: usize,
        s: Bytes,
        t: Bytes,
    };
    if &s == &t[..n] && &s == &t[m - n..] {
        println!("0");
    } else if &s == &t[..n] {
        println!("1");
    } else if &s == &t[m - n..] {
        println!("2");
    } else {
        println!("3");
    }
}
