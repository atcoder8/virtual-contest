use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        aa: [usize; h * w],
    }

    let min = *aa.iter().min().unwrap();
    let ans = aa.iter().sum::<usize>() - h * w * min;
    println!("{}", ans);
}
