// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

const T: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn main() {
    input! {
        c: [[u8; 3]; 3],
    };
    let mut cnt = 0usize;
    let mut tot = 0usize;
    for p in (0..9).permutations(9) {
        let mut f = true;
        for t in &T {
            let mut x = p.iter().filter(|p| t.contains(p));
            let a = x.next().unwrap();
            let b = x.next().unwrap();
            if c[a / 3][a % 3] == c[b / 3][b % 3] {
                f = false;
            }
        }
        if f {
            cnt += 1;
        }
        tot += 1;
    }
    println!("{}", cnt as f64 / tot as f64);
}
