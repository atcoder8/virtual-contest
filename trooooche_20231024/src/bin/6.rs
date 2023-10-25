use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
    }

    let mut ll = (1..=n / 2).filter(|&i| i != x).collect_vec();
    let mut rr = (n / 2 + 1..=n).filter(|&i| i != x).collect_vec();
    if ll.len() + 1 < rr.len() {
        ll.push(rr.remove(0));
    }
    ll.reverse();

    let mut pp = if ll.len() >= rr.len() {
        ll.into_iter().interleave(rr).collect_vec()
    } else {
        rr.into_iter().interleave(ll).collect_vec()
    };
    pp.insert(0, x);

    println!("{}", pp.iter().join(" "));
}
