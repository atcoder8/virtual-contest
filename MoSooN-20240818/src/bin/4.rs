use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut counts1 = vec![0_usize; n];
    let mut counts2 = vec![0_usize; n];
    for &a in &aa {
        counts2[a] += 1;
    }

    let mut ans = 0_usize;
    let mut sum_comb = 0_usize;
    for &a in &aa {
        let cnt1 = &mut counts1[a];
        let cnt2 = &mut counts2[a];

        sum_comb -= *cnt1 * *cnt2;

        ans += sum_comb;

        *cnt1 += 1;
        *cnt2 -= 1;
        sum_comb += *cnt1 * *cnt2;
    }
    println!("{}", ans);
}
