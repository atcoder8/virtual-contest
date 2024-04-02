use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        mut s: Chars,
        (a, b): (Usize1, Usize1),
    }

    s.swap(a, b);

    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
