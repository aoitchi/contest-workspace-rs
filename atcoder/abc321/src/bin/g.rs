// https://github.com/aoitchi/cplib-rs-old
// Bundled using https://github.com/aoitchi/contest-workspace-rs

use cplib::algebra::canonical::Canonical;
use cplib::bitwise::set_power_series::elementary::sps_log;
use cplib::combinatorics::factorial::Factorial;
use proconio::input;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};

use cplib::num::fp::{Fp, fp};

const P: u32 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
        r: [Usize1; m],
        b: [Usize1; m],
    };

    let fac = Factorial::<Canonical<Fp<P>>>::new(Canonical::new(), 1 << 20);

    let mut x = vec![0usize; n];
    for &r in &r {
        x[r] += 1;
    }
    let mut y = vec![0usize; n];
    for &b in &b {
        y[b] += 1;
    }

    let mut f = vec![];
    for v in 0..1 << n {
        let mut px = 0;
        let mut py = 0;
        for i in 0..n {
            if (v >> i) & 1 == 1 {
                px += x[i];
                py += y[i];
            }
        }
        if px != py {
            f.push(fp!(0, mod P));
        } else {
            f.push(fac.factorial(px));
        }
    }
    let g = sps_log(&Canonical::new(), &f);
    let mut cnt = fp!(0);
    for v in 0..1 << n {
        let w = (1 << n) - 1 - v;
        cnt += f[v] * g[w];
    }
    println!("{}", cnt / fac.factorial(m));
}
