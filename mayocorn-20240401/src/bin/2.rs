use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut count = ss.iter().sorted_unstable().dedup_with_count().collect_vec();
    let max_cnt = count.iter().map(|v| v.0).max().unwrap();
    count.retain(|v| v.0 == max_cnt);

    let ans = count.iter().map(|v| v.1).sorted_unstable().join("\n");
    println!("{}", ans);
}
