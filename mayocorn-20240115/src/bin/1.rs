use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n - 1],
    }

    let mut ans = 0;
    let mut cur = n - 1;
    while cur != 0 {
        cur = pp[cur - 1];
        ans += 1;
    }

    println!("{}", ans);
}
