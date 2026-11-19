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
        mut t: Bytes,
        mut s: [Bytes; n],
    };

    let mut a = vec![vec![], vec![]];
    for p in 0..2 {
        for s in s.iter() {
            let mut i = 0;
            for c in s.iter() {
                if i < t.len() && c == &t[i] {
                    i += 1;
                }
            }
            a[p].push(i);
        }
        t.reverse();
        for s in s.iter_mut() {
            s.reverse();
        }
    }

    let x = a[0].clone();
    let mut y = a[1].clone();

    y.sort();
    let mut ans = 0;
    for x in x.iter() {
        let i = y.partition_point(|y| x + y < t.len());
        ans += n - i;
    }
    println!("{ans}");
}
