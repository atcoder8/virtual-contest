use im_rc::HashSet;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (_n, q): (usize, usize),
        tab: [(usize, Usize1, Usize1); q],
    }

    let mut set = HashSet::<(usize, usize)>::new();
    for &(t, a, b) in &tab {
        match t {
            1 => {
                set.insert((a, b));
            }

            2 => {
                set.remove(&(a, b));
            }

            3 => {
                let ans = set.contains(&(a, b)) && set.contains(&(b, a));
                println!("{}", if ans { "Yes" } else { "No" });
            }

            _ => unreachable!(),
        }
    }
}
