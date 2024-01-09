use proconio::input;

fn main() {
    input! {
        n: usize,
        mut vv: [f64; n],
    }

    vv.sort_unstable_by(|x, y| x.partial_cmp(&y).unwrap());

    let mut ans = vv[0];
    for &v in &vv[1..] {
        ans = (ans + v) / 2.0;
    }
    println!("{}", ans);
}
