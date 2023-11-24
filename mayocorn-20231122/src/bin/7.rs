use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        aa: [usize; n],
    }

    let mut less = 0;
    let mut equal = 1;

    for window in aa.windows(2) {
        let rate = window[1] / window[0];
        let d = x % window[1] / window[0];

        (less, equal) = (
            less + (d > 0) as usize * equal,
            equal + (d < rate - 1) as usize * less,
        );
    }

    println!("{}", less + equal);
}
