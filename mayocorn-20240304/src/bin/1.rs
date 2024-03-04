use proconio::input;

fn main() {
    input! {
        n: usize,
        bb: [usize; n - 1],
    }

    let mut ans = 0;
    for i in 0..n {
        let mut lim_a = 10_usize.pow(5);
        if i > 0 {
            lim_a = lim_a.min(bb[i - 1]);
        }
        if i < n - 1 {
            lim_a = lim_a.min(bb[i]);
        }

        ans += lim_a;
    }

    println!("{}", ans);
}
