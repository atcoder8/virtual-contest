use proconio::input;

fn main() {
    input! {
        n: usize,
        ww: [usize; n],
    }

    let sum_w = ww.iter().sum::<usize>();

    let mut left = 0;
    let mut right = sum_w;

    let mut ans = sum_w;
    for &w in &ww {
        left += w;
        right -= w;

        ans = ans.min(left.abs_diff(right));
    }

    println!("{}", ans);
}
