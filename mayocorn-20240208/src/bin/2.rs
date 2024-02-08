use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
        bb: [Usize1; n],
        cc: [Usize1; n],
    }

    let mut counts = vec![0; n];
    for &c in &cc {
        counts[bb[c]] += 1;
    }

    let ans = aa.iter().map(|&a| counts[a]).sum::<usize>();
    println!("{}", ans);
}
