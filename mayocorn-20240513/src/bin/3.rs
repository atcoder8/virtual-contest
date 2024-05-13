use proconio::input;

const MAX: usize = 10_usize.pow(9);

fn main() {
    input! {
        (a, b, x): (usize, usize, usize),
    }

    let mut ok = 0_usize;
    let mut ng = MAX + 1;

    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if a * mid + b * mid.to_string().len() <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
