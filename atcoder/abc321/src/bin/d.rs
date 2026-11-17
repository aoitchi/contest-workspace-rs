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
        p: u64,
        mut a: [u64; n],
        mut b: [u64; m],
    };
    b.sort();
    let mut sb = vec![0];
    for &b in &b {
        sb.push(sb.last().unwrap() + b);
    }
    let mut ans = 0u64;
    for &a in &a {
        let k = b.partition_point(|b| a + b < p);
        ans += a * k as u64 + sb[k] + (m - k) as u64 * p;
    }
    println!("{}", ans);
}
