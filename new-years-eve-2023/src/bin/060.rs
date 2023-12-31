use proconio::input;

fn main() {
    input! {
        n: usize,
        pp: [usize; n],
    }

    let ans = pp.iter().sum::<usize>() - pp.iter().max().unwrap() / 2;
    println!("{}", ans);
}
