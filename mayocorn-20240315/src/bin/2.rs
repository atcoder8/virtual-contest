use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut mat = Array2::from_elem((n, n), false);
    for &(u, v) in &uv {
        mat[(u, v)] = true;
        mat[(v, u)] = true;
    }

    let mut ans = 0;
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                ans += (mat[(a, b)] && mat[(b, c)] && mat[(c, a)]) as usize;
            }
        }
    }
    println!("{}", ans);
}
