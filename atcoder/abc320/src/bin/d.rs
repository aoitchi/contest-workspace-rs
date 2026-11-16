// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

const INF: i64 = 1 << 62;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1, i64, i64); m],
    };

    let mut e = vec![vec![]; n];
    for &(a, b, x, y) in &ab {
        e[a].push((b, x, y));
        e[b].push((a, -x, -y));
    }

    let mut ans = vec![(INF, INF); n];
    ans[0] = (0, 0);
    let mut stack = vec![0];
    while let Some(u) = stack.pop() {
        for &(v, x, y) in &e[u] {
            let (nx, ny) = (ans[u].0 + x, ans[u].1 + y);
            if ans[v] == (INF, INF) {
                ans[v] = (nx, ny);
                stack.push(v);
            }
        }
    }
    for &(x, y) in ans.iter() {
        if x == INF {
            println!("undecidable");
        } else {
            println!("{} {}", x, y);
        }
    }
}
