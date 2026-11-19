// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        s: Bytes,
    };
    let mut ans = true;
    for i in 1..16 {
        if i & 1 == 1 {
            if s[i] != b'0' {
                ans = false;
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
