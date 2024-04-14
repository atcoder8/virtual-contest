use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut flags = vec![false; x + 1];
    flags[1] = true;
    for base in 2..=x {
        for exp in 2.. {
            let val = base.pow(exp);

            if val > x {
                break;
            }

            flags[val] = true;
        }
    }

    let ans = (1..=x).rev().find(|&val| flags[val]).unwrap();
    println!("{}", ans);
}
