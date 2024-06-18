use proconio::input;

fn main() {
    input! {
        mut aa: [usize; 3],
    }

    aa.sort_unstable();
    let ans = aa[2] - aa[1] == aa[1] - aa[0];
    println!("{}", if ans { "Yes" } else { "No" });
}
