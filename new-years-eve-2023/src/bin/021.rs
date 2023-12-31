use proconio::input;

fn main() {
    input! {
        n: usize,
        dd: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans += dd[i] * dd[j];
        }
    }
    println!("{}", ans);
}
