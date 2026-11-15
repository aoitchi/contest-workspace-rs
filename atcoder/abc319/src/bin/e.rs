// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

const T: usize = 2 * 2 * 2 * 3 * 5 * 7;

fn main() {
    input! {
        n: usize,
        x: u64,
        y: u64,
        pt: [(u64, u64); n - 1],
        q: usize,
        queries: [usize; q],
    };

    let mut v = [0; T];
    for i in 0..T {
        v[i] = i as u64 + x;
    }
    for &(p, t) in &pt {
        for i in 0..T {
            v[i] = v[i].next_multiple_of(p) + t;
        }
    }
    for i in 0..T {
        v[i] += y;
    }

    for q in &queries {
        println!("{}", (q / T * T) as u64 + v[q % T]);
    }
}
