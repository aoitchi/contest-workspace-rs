// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use std::collections::BinaryHeap;

use cplib::collections::binary_trie::BinaryTrie;
use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(u64, u64, u64); m],
    };
    let mut heap = BinaryHeap::new();
    let mut trie = BinaryTrie::new(30);
    for i in 0..n {
        trie.insert(i as u64);
    }
    for &(t, w, s) in &tws {
        heap.push((!t, 0, w, s));
    }

    let mut ans = vec![0; n];
    while let Some((t, k, w, s)) = heap.pop() {
        let t = !t;
        if k == 0 {
            let x = trie.min();
            if let Some(x) = x {
                ans[x as usize] += w;
                trie.remove(x);
                heap.push((!(t + s), 1, x, !0));
            }
        } else {
            trie.insert(w);
        }
    }

    println!("{}", ans.iter().join("\n"));
}
