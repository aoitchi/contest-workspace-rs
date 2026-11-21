// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use std::collections::HashMap;

use cplib::collections::dsu::Dsu;
use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Bytes; h],
    };
    let mut map = HashMap::new();
    let mut cnt = 0usize;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'#' {
                map.insert((i, j), cnt);
                cnt += 1;
            }
        }
    }

    let mut dsu = Dsu::new(cnt);
    for (&(i, j), &u) in &map {
        for (di, dj) in (-1..=1).cartesian_product(-1..=1) {
            let ni = i.wrapping_add_signed(di);
            let nj = j.wrapping_add_signed(dj);
            if let Some(&v) = map.get(&(ni, nj)) {
                dsu.unite(u, v);
            }
        }
    }
    println!("{}", dsu.num_sets());
}
