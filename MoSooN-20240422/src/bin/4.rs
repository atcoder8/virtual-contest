use itertools::{chain, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, k): (usize, usize),
        s: Chars,
    }

    let s: Vec<usize> = s
        .iter()
        .map(|&c| match c {
            'R' => 0,
            'P' => 1,
            'S' => 2,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut hands = (0..2 * n).map(|i| s[i % n]).collect_vec();
    for _ in 0..k {
        let next_hands = hands
            .chunks_mut(2)
            .map(|hand| {
                hand.sort_unstable();

                let hand1 = hand[0];
                let hand2 = hand[1];

                if hand1 == hand2 {
                    return hand1;
                }

                match (hand1, hand2) {
                    (0, 1) => 1,
                    (0, 2) => 0,
                    (1, 2) => 2,
                    _ => unreachable!(),
                }
            })
            .collect_vec();
        hands = chain(&next_hands, &next_hands).cloned().collect();
    }

    let ans = match hands[0] {
        0 => 'R',
        1 => 'P',
        2 => 'S',
        _ => unreachable!(),
    };
    println!("{}", ans);
}
