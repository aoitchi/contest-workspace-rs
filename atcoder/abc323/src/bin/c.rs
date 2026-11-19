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
        mut a: [usize; m],
        s: [Bytes; n],
    };
    let mut t = vec![];
    for (i, s) in s.iter().enumerate() {
        let x = (0..m)
            .map(|i| if s[i] == b'o' { a[i] } else { 0 })
            .sum::<usize>();
        t.push(x + i);
    }
    let mut idx: Vec<usize> = (0..m).collect();
    idx.sort_by_key(|i| !a[*i]);
    for i in 0..n {
        let mut x = t[i];
        let mut mx = 0;
        for j in 0..n {
            if j != i {
                mx = mx.max(t[j]);
            }
        }
        let mut ans = 0usize;
        if x > mx {
            println!("0");
        } else {
            for &j in idx.iter() {
                if s[i][j] == b'o' {
                    continue;
                }
                x += a[j];
                ans += 1;
                if x > mx {
                    break;
                }
            }
            println!("{ans}");
        }
    }
}
