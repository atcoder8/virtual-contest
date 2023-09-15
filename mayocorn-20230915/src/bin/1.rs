use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        ss: [char; 3],
        tt: [char; 3],
    }

    let positions = ss
        .iter()
        .map(|s| tt.iter().position(|t| t == s).unwrap())
        .collect_vec();
    let mut cnt = 0;
    for i in 0..3 {
        for j in 0..i {
            cnt += (positions[i] < positions[j]) as usize;
        }
    }

    println!("{}", if cnt % 2 == 0 { "Yes" } else { "No" });
}
