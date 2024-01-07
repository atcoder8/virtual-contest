use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let counts = aa.iter().cloned().sorted_unstable().dedup_with_count();
    let mut rem_k = k;
    let mut rem_basket = n;
    let mut cycle_num = 0;
    let mut prev = 0;
    for (cnt, cur) in counts {
        if rem_basket * (cur - prev) >= rem_k {
            cycle_num += rem_k / rem_basket;
            rem_k %= rem_basket;

            break;
        }

        rem_k -= rem_basket * (cur - prev);
        rem_basket -= cnt;
        cycle_num += cur - prev;
        prev = cur;
    }

    let mut bb = aa.iter().map(|&a| {
        if a <= cycle_num {
            0
        } else if rem_k != 0 {
            rem_k -= 1;

            a - cycle_num - 1
        } else {
            a - cycle_num
        }
    });
    println!("{}", bb.join(" "));
}
