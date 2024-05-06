use itertools::izip;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        s: String,
        ww: [usize; n],
    }

    let mut children = vec![];
    let mut adults = vec![];
    for (parity, &w) in izip!(s.chars(), &ww) {
        if parity == '0' {
            children.push(w);
        } else {
            adults.push(w);
        }
    }
    children.sort_unstable();
    adults.sort_unstable();

    let mut max_score = children.len();
    for &threshold in &ww {
        let score =
            children.lower_bound(&threshold) + adults.len() - adults.lower_bound(&threshold);
        max_score = max_score.max(score);
    }
    println!("{}", max_score);
}
