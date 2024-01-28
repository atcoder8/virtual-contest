use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut counts = vec![0; 8];
    let mut free_cnt = 0;
    for &a in &aa {
        if a >= 3200 {
            free_cnt += 1;
        } else {
            counts[a / 400] += 1;
        }
    }

    let base = counts.iter().filter(|&&cnt| cnt != 0).count();

    let min = base.max(1);
    let max = base + free_cnt;
    println!("{} {}", min, max);
}
