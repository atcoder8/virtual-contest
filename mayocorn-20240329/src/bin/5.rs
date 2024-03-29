use proconio::input;

const MOD: usize = 998244353;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, k, m): (usize, usize, usize),
    }

    if m % MOD == 0 {
        return 0;
    }

    let exp = pow_mod(k, n, MOD - 1);
    pow_mod(m, exp, MOD)
}

/// Calculate the remainder of `exp` power of `base` divided by `m`.
pub fn pow_mod(base: usize, exp: usize, m: usize) -> usize {
    let mut ret = 1 % m;
    let mut mul = base % m;
    let mut t = exp;

    while t != 0 {
        if t & 1 == 1 {
            ret = ret * mul % m;
        }

        mul = mul * mul % m;
        t >>= 1;
    }

    ret
}
