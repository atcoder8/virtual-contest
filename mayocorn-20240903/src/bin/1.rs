use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n: i64,
    }

    let ans = (n % MOD + MOD) % MOD;
    println!("{}", ans);
}
