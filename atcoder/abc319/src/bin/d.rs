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
        a: [usize; n],
    };

    // (l, r]
    let mut r = 1 << 63;
    let mut l = a.iter().max().unwrap() - 1;
    while r - l > 1 {
        let o = (r + l) / 2;
        let mut cnt = 1;
        let mut buf = 0;
        for &a in &a {
            let d = if buf == 0 { a } else { a + 1 };
            if buf + d <= o {
                buf += d;
            } else {
                buf = a;
                cnt += 1;
            }
        }
        if cnt <= m {
            r = o;
        } else {
            l = o;
        }
    }
    println!("{r}");
}
