// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        m: usize,
        s: [Bytes; 3],
    };

    let mut ans = !0;
    for i in 0..=3 * m {
        for j in 0..=3 * m {
            for k in 0..=3 * m {
                if i != j
                    && j != k
                    && k != i
                    && s[0][i % m] == s[1][j % m]
                    && s[1][j % m] == s[2][k % m]
                {
                    ans = ans.min(i.max(j).max(k));
                }
            }
        }
    }
    println!("{}", ans as isize);
}
