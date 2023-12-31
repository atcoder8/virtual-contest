use proconio::input;

fn main() {
    input! {
        mut seq: [usize; 3],
    }

    seq.sort_unstable();

    let ans = seq == vec![5, 5, 7];
    println!("{}", if ans { "YES" } else { "NO" });
}
