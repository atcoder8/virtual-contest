use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        m: usize,
        bb: [usize; m],
        x: usize,
    }

    let mut mochi = vec![false; x];
    for &b in &bb {
        mochi[b] = true;
    }
    let mut dp = vec![false; x + 1];
    dp[0] = true;

    for i in 0..x {
        if mochi[i] || !dp[i] {
            continue;
        }

        for &a in &aa {
            if i + a > x {
                break;
            }

            dp[i + a] = true;
        }
    }

    println!("{}", if dp[x] { "Yes" } else { "No" });
}
