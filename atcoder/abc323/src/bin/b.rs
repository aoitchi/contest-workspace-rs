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
        s: [Bytes; n],
    };
    let mut cnt = vec![0usize; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == b'o' {
                cnt[i] += 1;
            }
        }
    }
    let mut ans: Vec<usize> = (0..n).collect();
    ans.sort_by_key(|&i| (!cnt[i], i));
    println!("{}", ans.iter().map(|i| i + 1).join(" "));
}
