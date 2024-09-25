use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut ans = n;
    let mut left = 0;
    while left + 1 < n {
        let diff = aa[left + 1] - aa[left];
        let mut right = left + 1;
        while right + 1 < n && aa[right + 1] - aa[right] == diff {
            right += 1;
        }

        let dist = right - left + 1;
        ans += dist * (dist - 1) / 2;
        left = right;
    }
    println!("{}", ans);
}
