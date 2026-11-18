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
        a: [usize; m],
    };
    let mut ans = vec![!0usize; n + 1];
    for &a in &a {
        ans[a] = 0;
    }
    for i in (0..n).rev() {
        ans[i] = ans[i].min(ans[i + 1] + 1);
    }
    println!("{}", ans[1..].iter().join("\n"));
}
