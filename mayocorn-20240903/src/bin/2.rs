use fixedbitset::FixedBitSet;
use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [String; n],
    }

    let bits_each_store = ss
        .iter()
        .map(|s| {
            let mut bitset = FixedBitSet::with_capacity(m);
            for (i, c) in enumerate(s.chars()) {
                bitset.set(i, c == 'o');
            }
            bitset
        })
        .collect_vec();

    let is_ok = |bits: usize| {
        let mut union = FixedBitSet::with_capacity(m);
        for i in 0..n {
            if bits >> i & 1 == 1 {
                union.union_with(&bits_each_store[i]);
            }
        }

        union.count_ones(..) == m
    };

    let ans = (0..1 << n)
        .filter_map(|bits| {
            if is_ok(bits) {
                Some(bits.count_ones())
            } else {
                None
            }
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
