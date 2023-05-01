use itertools::join;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![vec![0; n]; n];
    let mut cnt = 0;

    for i in (0..n).step_by(2) {
        for j in 0..n {
            cnt += 1;
            ans[i][j] = cnt;
        }
    }

    for i in (1..n).step_by(2) {
        for j in 0..n {
            cnt += 1;
            ans[i][j] = cnt;
        }
    }

    for line in &ans {
        println!("{}", join(line, " "));
    }
}
