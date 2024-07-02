use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut aa: [usize; n],
    }

    let mut counter = aa
        .iter()
        .filter(|&&a| a != 0)
        .cloned()
        .sorted_unstable()
        .dedup_with_count()
        .collect::<VecDeque<_>>();

    let mut rem_basket_num = aa.iter().filter(|&&a| a != 0).count();
    let mut rem = k;
    let mut offset = 0;

    loop {
        let (cnt, origin_num) = counter.pop_front().unwrap();
        let apple_num_each_basket = origin_num - offset;

        let take = rem.min(rem_basket_num * apple_num_each_basket);
        let (q, r) = (take / rem_basket_num, take % rem_basket_num);

        if take == rem {
            aa.iter_mut()
                .for_each(|a| *a = a.saturating_sub(offset + q));
            aa.iter_mut()
                .filter(|a| **a != 0)
                .take(r)
                .for_each(|a| *a -= 1);

            break;
        }

        rem_basket_num -= cnt;
        rem -= take;
        offset = origin_num;
    }

    println!("{}", aa.iter().join(" "));
}
