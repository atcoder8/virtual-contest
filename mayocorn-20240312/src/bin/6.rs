use ac_library::ModInt as Mint;
use proconio::input;

fn main() {
    input! {
        (n, p): (usize, u32),
    }

    Mint::set_modulus(p);

    // dp[i][j]: 置き換え後の文字数がiで置き換え前の文字数がj
    let mut dp = vec![vec![Mint::new(0); n + 1]; n];
    dp[0][0] = Mint::new(1);

    for replaced_len in 0..n - 1 {
        // 10^digit_num 以上 10^(digit_num+1) 未満の長さだけ同じ文字を続ける
        for digit_num in 1..=4 {
            let next_replaced_len = replaced_len + digit_num + 1;

            if next_replaced_len >= n {
                continue;
            }

            let mut imos = vec![Mint::new(0); n + 2];
            for len in 0..n {
                // 区間[len+10^d, len+10^(d-1))に加算される
                let add = dp[replaced_len][len] * (25 + (replaced_len == 0) as usize);
                let left = (len + 10_usize.pow(digit_num as u32 - 1)).min(n + 1);
                let right = (len + 10_usize.pow(digit_num as u32)).min(n + 1);

                imos[left] += add;
                imos[right] -= add;
            }
            for i in 0..=n {
                let add = imos[i];
                imos[i + 1] += add;
            }

            for next_len in 0..=n {
                dp[next_replaced_len][next_len] += imos[next_len];
            }
        }
    }

    let mut ans = Mint::new(0);
    for replaced_len in 0..n {
        ans += dp[replaced_len][n];
    }
    println!("{}", ans);
}
