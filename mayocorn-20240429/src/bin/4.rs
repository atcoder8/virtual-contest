use proconio::input;

const MAX: usize = 10_usize.pow(6);

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut counts = vec![0; MAX + 1];
    for &a in &aa {
        counts[a] += 1;
    }

    for i in 1..=MAX {
        if counts[i] == 0 {
            continue;
        }

        for j in (2 * i..=MAX).step_by(i) {
            counts[j] = 0;
        }
    }

    let ans = counts.iter().filter(|&&cnt| cnt == 1).count();
    println!("{}", ans);
}
