use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut ans = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && aa[right] < aa[left] + m {
            right += 1;
        }

        ans = ans.max(right - left);
    }

    println!("{}", ans);
}
