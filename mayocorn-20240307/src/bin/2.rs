use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        hands_per_player: [Chars; 2 * n],
    }

    let player_num = 2 * n;

    let compete = |win_counts: &mut [usize], round: usize, orders: &mut [usize]| {
        for chunk in orders.chunks(2) {
            let p1 = chunk[0];
            let p2 = chunk[1];

            win_counts[p1] +=
                win(hands_per_player[p1][round], hands_per_player[p2][round]) as usize;
            win_counts[p2] +=
                win(hands_per_player[p2][round], hands_per_player[p1][round]) as usize;
        }

        orders.sort_unstable_by_key(|&player| (Reverse(win_counts[player]), player));
    };

    let mut win_counts = vec![0; player_num];
    let mut orders = (0..player_num).collect_vec();

    (0..m).for_each(|round| compete(&mut win_counts, round, &mut orders));

    let ans = orders.iter().map(|order| order + 1).join("\n");
    println!("{}", ans);
}

fn win(hand: char, other_hand: char) -> bool {
    let lose = match hand {
        'G' => 'C',
        'C' => 'P',
        'P' => 'G',
        _ => unreachable!(),
    };

    other_hand == lose
}
