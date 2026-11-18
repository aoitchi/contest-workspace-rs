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
        s: Bytes,
    };
    let mut ans = !0usize;
    for i in 0..s.len() - 2 {
        if s[i..i + 3] == [b'A', b'B', b'C'] {
            ans = ans.min(i + 1);
        }
    }
    println!("{}", ans as isize);
}
