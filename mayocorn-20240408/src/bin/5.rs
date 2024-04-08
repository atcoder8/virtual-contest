use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> u128 {
    input! {
        (a, x, m): (u128, u128, u128),
    }

    if a == 1 {
        return x % m;
    }

    (pow_mod(a, x, (a - 1) * m) - 1) / (a - 1) % m
}

/// Calculate the remainder of `exp` power of `base` divided by `m`.
pub fn pow_mod(base: u128, exp: u128, m: u128) -> u128 {
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
