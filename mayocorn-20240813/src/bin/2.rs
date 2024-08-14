use proconio::input;

fn main() {
    input! {
        (d, n): (usize, usize),
    }

    let is_ok = |mut val: usize| {
        for _ in 0..d {
            if val % 100 != 0 {
                return false;
            }

            val /= 100;
        }

        val % 100 != 0
    };

    let ans = (1..).filter(|&val| is_ok(val)).nth(n - 1).unwrap();
    println!("{}", ans);
}
