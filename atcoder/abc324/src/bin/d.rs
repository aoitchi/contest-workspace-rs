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
        s: String,
    };
    let mut a = vec![0usize; 10];
    for c in s.chars() {
        a[c as usize - 48] += 1;
    }

    let mut ans = 0usize;
    for i in 0usize..1 << 25 {
        let x = (i * i).to_string();
        let mut t = vec![0usize; 10];
        for c in x.chars() {
            t[c as usize - 48] += 1;
        }
        if a[0] >= t[0] && (1..10).all(|i| a[i] == t[i]) {
            ans += 1;
        }
    }
    println!("{ans}");
}
