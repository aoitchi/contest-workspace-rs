// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

fn main() {
    input! {
        t: (i64, i64),
        u: (i64, i64),
        v: (i64, i64),
    };

    let mut ans = !0;

    let mut k = 0;
    let a = if u.0 < v.0 {
        if t.1 == u.1 && u.0 < t.0 {
            k = 2;
        }
        (u.0 - 1, u.1)
    } else {
        if t.1 == u.1 && u.0 > t.0 {
            k = 2;
        }
        (u.0 + 1, u.1)
    };
    let b = if u.0 < v.0 {
        (v.0 - 1, u.1)
    } else {
        (v.0 + 1, u.1)
    };
    if u.1 == v.1 {
        ans = ans.min(d(t, a) + d(a, b) + k);
    } else {
        ans = ans.min(d(t, a) + d(a, b) + 2 + u.1.abs_diff(v.1) + k);
    }

    let mut k = 0;
    let a = if u.1 < v.1 {
        if t.0 == u.0 && u.1 < t.1 {
            k = 2;
        }
        (u.0, u.1 - 1)
    } else {
        if t.0 == u.0 && u.1 > t.1 {
            k = 2;
        }
        (u.0, u.1 + 1)
    };
    let b = if u.1 < v.1 {
        (u.0, v.1 - 1)
    } else {
        (u.0, v.1 + 1)
    };
    if u.0 == v.0 {
        ans = ans.min(d(t, a) + d(a, b) + k);
    } else {
        ans = ans.min(d(t, a) + d(a, b) + 2 + u.0.abs_diff(v.0) + k);
    }

    println!("{ans}");
}

fn d(a: (i64, i64), b: (i64, i64)) -> u64 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}
