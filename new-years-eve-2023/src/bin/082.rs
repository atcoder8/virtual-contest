use proconio::input;

fn main() {
    input! {
        n: usize,
        (a, b): (usize, usize),
        pp: [usize; n],
    }

    let cnt1 = pp.iter().filter(|&&p| p <= a).count();
    let cnt2 = pp.iter().filter(|&&p| a < p && p <= b).count();
    let cnt3 = pp.iter().filter(|&&p| b < p).count();

    let ans = cnt1.min(cnt2).min(cnt3);
    println!("{}", ans);
}
