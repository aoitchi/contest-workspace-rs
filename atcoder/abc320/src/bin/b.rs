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

    let mut ans = 0usize;
    for i in 0..s.len() {
        for j in i..=s.len() {
            let t = &s[i..j].to_vec();
            let mut u = t.clone();
            u.reverse();
            if *t == u {
                ans = ans.max(t.len());
            }
        }
    }
    println!("{ans}");
}
