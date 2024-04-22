use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [usize; h * w],
    }

    let min = *aaa.iter().min().unwrap();
    let ans = aaa.iter().map(|a| a - min).sum::<usize>();
    println!("{}", ans);
}
