use proconio::input;

fn main() {
    input! {
        n: usize,
        xx: [usize; n],
    }

    let calc_cost = |p: usize| xx.iter().map(move |x| x.abs_diff(p).pow(2)).sum::<usize>();

    let ans = (1..=100).map(calc_cost).min().unwrap();
    println!("{}", ans);
}
