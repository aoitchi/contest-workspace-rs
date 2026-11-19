// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use std::collections::BinaryHeap;

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        n: usize,
        sc: [(u64, usize); n],
    };
    let mut heap = BinaryHeap::new();
    for &(s, c) in &sc {
        heap.push((!s, c));
    }
    let mut ans = 0usize;
    while let Some((s, c)) = heap.pop() {
        let mut c = c;
        while heap.len() > 0 && heap.peek().unwrap().0 == s {
            let (_, cc) = heap.pop().unwrap();
            c += cc;
        }
        let s = !s;
        ans += c % 2;
        if c / 2 > 0 {
            heap.push((!(2 * s), c / 2));
        }
    }
    println!("{ans}");
}
