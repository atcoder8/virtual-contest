use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        pp: [usize; n],
    }

    let half_n = (n - 1) / 2;

    let mut counts = vec![0; 2 * n];

    let mut sum_cost = 0;
    let mut pos_cnt = 0;
    let mut neg_cnt = 0;

    for (i, &p) in enumerate(&pp) {
        let diff = (n + p - i) % n;

        counts[diff] += 1;
        counts[diff + n] += 1;

        if diff <= half_n {
            sum_cost += diff;
            pos_cnt += 1;
        } else {
            sum_cost += n - diff;
            neg_cnt += 1;
        }
    }

    let mut min_sum_cost = sum_cost;
    for shift in 1..n {
        let border_pos_cnt = counts[n + half_n - (shift - 1)];
        let border_neg_cnt = counts[n - 1 - (shift - 1)];

        pos_cnt -= border_pos_cnt;
        neg_cnt -= border_neg_cnt;

        sum_cost += pos_cnt;
        sum_cost -= neg_cnt;

        neg_cnt += border_pos_cnt;
        pos_cnt += border_neg_cnt;

        sum_cost += (n - 1) % 2 * border_pos_cnt;
        sum_cost -= border_neg_cnt;

        min_sum_cost = min_sum_cost.min(sum_cost);
    }

    println!("{}", min_sum_cost);
}
