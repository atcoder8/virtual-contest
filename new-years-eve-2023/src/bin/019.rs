use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        tt: [usize; n],
        m: usize,
        px: [(Usize1, usize); m],
    }

    let sum = tt.iter().sum::<usize>();

    for &(p, x) in &px {
        println!("{}", sum + x - tt[p]);
    }
}
