use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        (n, max_channel_num): (usize, usize),
        mut stc: [(usize, usize, usize); n],
    }

    stc.sort_unstable_by_key(|x| x.0);

    let mut ans = 0;
    let mut recorder_set = BTreeSet::<(usize, usize)>::new();
    for &(s, t, c) in &stc {
        let recorder = if recorder_set.contains(&(s, c)) {
            Some((s, c))
        } else {
            recorder_set
                .range(..=(s, max_channel_num))
                .take(max_channel_num)
                .find(|&&(term, _channel)| term < s)
                .cloned()
        };

        match recorder {
            Some(recorder) => {
                recorder_set.remove(&recorder);
            }
            None => {
                ans += 1;
            }
        };

        recorder_set.insert((t, c));
    }

    println!("{}", ans);
}
