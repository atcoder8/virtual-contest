use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, _w): (usize, usize),
        sss: [Chars; h],
    }

    let ans = sss.iter().flatten().filter(|&&c| c == '#').count();
    println!("{}", ans);
}
