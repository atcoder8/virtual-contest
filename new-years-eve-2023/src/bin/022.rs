use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut ll: [usize; n],
    }

    ll.sort_unstable();

    let ans = ll.iter().rev().take(k).sum::<usize>();
    println!("{}", ans);
}
