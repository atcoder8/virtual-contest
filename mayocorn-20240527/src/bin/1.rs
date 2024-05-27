use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut sum = 0;
    let mut low_lim = 0;

    for &a in &aa {
        if a < low_lim {
            sum += low_lim - a;
        } else {
            low_lim = low_lim.max(a);
        }
    }

    println!("{}", sum);
}
