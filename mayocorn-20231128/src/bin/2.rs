use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }

    let (counts1, at_num_1) = count_chars(&s);
    let (counts2, at_num_2) = count_chars(&t);

    let solve = || {
        let mut rem1 = at_num_1;
        let mut rem2 = at_num_2;

        for i in 0..26 {
            let cnt1 = counts1[i];
            let cnt2 = counts2[i];

            if cnt1 == cnt2 {
                continue;
            }

            if !"atcoder".contains((b'a' + i as u8) as char) {
                return false;
            }

            if cnt1 < cnt2 {
                if rem1 < cnt2 - cnt1 {
                    return false;
                }

                rem1 -= cnt2 - cnt1;
            } else if cnt1 > cnt2 {
                if rem2 < cnt1 - cnt2 {
                    return false;
                }

                rem2 -= cnt1 - cnt2;
            }
        }

        true
    };

    println!("{}", if solve() { "Yes" } else { "No" });
}

fn count_chars(s: &Vec<u8>) -> (Vec<usize>, usize) {
    let mut counts = vec![0; 26];
    let mut cnt_at = 0;

    for &c in s {
        if c == b'@' {
            cnt_at += 1;
        } else {
            counts[(c - b'a') as usize] += 1;
        }
    }

    (counts, cnt_at)
}
