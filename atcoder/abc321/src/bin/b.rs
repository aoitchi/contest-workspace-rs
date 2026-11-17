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
        x: u32,
        a: [u32; n - 1],
    };
    let mut ans = !0u32;
    for i in 0..=100 {
        let mut b = a.clone();
        b.push(i);
        let mn = b.iter().min().unwrap();
        let mx = b.iter().max().unwrap();
        if b.iter().sum::<u32>() - mn - mx >= x {
            ans = ans.min(i);
        }
    }
    println!("{}", ans as i32);
}
