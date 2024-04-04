use proconio::input;

fn main() {
    input! {
        (n, l, r): (usize, i64, i64),
        mut aa: [i64; n],
    }

    let mut sum = r * n as i64;
    let mut sum_diff = 0;
    let mut best_sum_diff = 0;
    let mut ans = sum;
    for i in 0..n {
        sum += aa[i] - r;
        sum_diff += l - aa[i];
        best_sum_diff = best_sum_diff.min(sum_diff);
        ans = ans.min(sum + best_sum_diff);
    }

    println!("{}", ans);
}
