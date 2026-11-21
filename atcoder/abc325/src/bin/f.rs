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
        d: [u64; n],
        lck: [(u64, usize, usize); 2],
    };
    // d[i]: センサー0をi個つかったときのセンサー1の個数の最小値
    let mut dp = vec![0; lck[0].2 + 1];
    for &d in &d {
        let mut ep = vec![!0; 1 << 10];
        for i in 0..=lck[0].2 {
            let j = if lck[0].0 * i as u64 >= d {
                0
            } else {
                (d - lck[0].0 * i as u64 + lck[1].0 - 1) / lck[1].0
            } as usize;
            for x in 0..=lck[0].2 {
                if x + i <= lck[0].2 {
                    ep[x + i] = ep[x + i].min(dp[x] + j);
                }
            }
        }
        dp = ep;
    }
    if *dp.iter().min().unwrap() > lck[1].2 {
        println!("-1");
    } else {
        let ans = dp
            .iter()
            .enumerate()
            .map(|(i, j)| {
                if *j <= lck[1].2 {
                    lck[0].1 * i + lck[1].1 * j
                } else {
                    !0
                }
            })
            .min()
            .unwrap();
        println!("{ans}");
    }
}
