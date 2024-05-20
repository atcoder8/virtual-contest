use ac_library::ModInt1000000007;
use itertools::Itertools;
use proconio::input;

type Mint = ModInt1000000007;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let aa = aa.iter().map(|&a| Mint::new(a)).collect_vec();

    let ans = (aa.iter().sum::<Mint>().pow(2) - aa.iter().map(|a| a.pow(2)).sum::<Mint>()) / 2;
    println!("{}", ans);
}
