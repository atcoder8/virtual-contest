use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; n],
        mut bb: [usize; m],
    }

    aa.sort_unstable();
    bb.sort_unstable();

    let mut iter_a = aa.iter();
    let ans = bb.iter().all(|&b| iter_a.find(|&&a| a == b).is_some());
    println!("{}", if ans { "Yes" } else { "No" });
}
