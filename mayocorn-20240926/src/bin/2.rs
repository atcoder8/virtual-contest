use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let counter = ss.iter().sorted_unstable().dedup_with_count().collect_vec();
    let max_count = counter.iter().map(|&(cnt, _)| cnt).max().unwrap();
    let ans = counter
        .iter()
        .filter_map(|&(cnt, s)| if cnt == max_count { Some(s) } else { None })
        .join("\n");
    println!("{}", ans);
}
