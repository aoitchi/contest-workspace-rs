// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use std::collections::HashMap;

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
    };
    let mut dp = HashMap::new();
    dp.insert(vec![0; k], 0);
    for _ in 0..n {
        input! {
            c: usize,
            a: [usize; k],
        }
        let mut ep = dp.clone();
        for (key, value) in dp.iter() {
            let mut key = key.clone();
            for i in 0..k {
                key[i] += a[i];
                key[i] = p.min(key[i]);
            }
            *ep.entry(key.clone()).or_insert(0) = *ep.get(&key).unwrap_or(&!0).min(&(*value + c));
        }
        dp = ep;
    }
    println!("{}", *dp.get(&vec![p; k]).unwrap_or(&!0) as isize);
}
