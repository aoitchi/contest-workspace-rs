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
        a: u64,
        b: u64,
        c: u64,
        e: [[u64; n]; n],
    };

    let mut heap = BinaryHeap::new();
    // dist[i] = [x, y]: 0 -> iへ社用車だけでx, 電車に乗り換えてy。
    let mut dist = vec![[!0, !0]; n];
    dist[0] = [0, 0];
    heap.push((!0, 0, 0));
    heap.push((!0, 0, 1));
    while let Some((d, i, k)) = heap.pop() {
        let d = !d;
        if dist[i][k] < d {
            continue;
        }
        for j in 0..n {
            // 社用車
            if k == 0 {
                let nd = d + e[i][j] * a;
                if nd < dist[j][0] {
                    heap.push((!nd, j, 0));
                    dist[j][0] = nd;
                }
            }
            // 電車
            let nd = d + e[i][j] * b + c;
            if nd < dist[j][1] {
                heap.push((!nd, j, 1));
                dist[j][1] = nd;
            }
        }
    }
    println!("{}", dist[n - 1].iter().min().unwrap());
}
