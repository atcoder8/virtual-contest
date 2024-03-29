use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut exists = vec![false; 8];
    let mut free_cnt = 0;
    for &a in &aa {
        if a >= 3200 {
            free_cnt += 1;
        } else {
            exists[a / 400] = true;
        }
    }

    let base = exists.iter().filter(|&&x| x).count();
    let min = base.max(1);
    let max = base + free_cnt;
    println!("{} {}", min, max);
}
