use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        costs: [usize; 3],
    }

    let ans = costs
        .into_iter()
        .tuple_combinations()
        .map(|(c1, c2)| c1 + c2)
        .min()
        .unwrap();
    println!("{}", ans);
}
