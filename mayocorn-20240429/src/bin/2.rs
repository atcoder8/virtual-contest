use itertools::iproduct;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, _m): (usize, usize),
        products: [(usize, [Usize1]); n],
    }

    let is_ok = |this: (usize, &[usize]), other: (usize, &[usize])| {
        if this.0 < other.0 {
            return false;
        }

        let this_bit = this
            .1
            .iter()
            .map(|features| 1_u128 << features)
            .sum::<u128>();
        let other_bit = other
            .1
            .iter()
            .map(|features| 1_u128 << features)
            .sum::<u128>();

        other_bit | this_bit == other_bit && (this.0 > other.0 || other_bit > this_bit)
    };

    let ans = iproduct!(&products, &products)
        .any(|(this, other)| is_ok((this.0, &this.1), (other.0, &other.1)));
    println!("{}", if ans { "Yes" } else { "No" });
}
