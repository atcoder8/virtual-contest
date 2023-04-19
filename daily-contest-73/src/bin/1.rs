use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    }

    let cards1 = s[..4]
        .iter()
        .map(|c| c.to_digit(10).unwrap() as usize - 1)
        .collect_vec();
    let cards2 = t[..4]
        .iter()
        .map(|c| c.to_digit(10).unwrap() as usize - 1)
        .collect_vec();

    let mut card_counts_1 = vec![0; 9];
    for &card in &cards1 {
        card_counts_1[card] += 1;
    }

    let mut card_counts_2 = vec![0; 9];
    for &card in &cards2 {
        card_counts_2[card] += 1;
    }

    let mut rem_cards = vec![k; 9];
    for card in 0..9 {
        rem_cards[card] -= card_counts_1[card] + card_counts_2[card];
    }

    let mut ans = 0.0;

    for back_card_1 in 0..9 {
        if rem_cards[back_card_1] == 0 {
            continue;
        }

        let prob1 = rem_cards[back_card_1] as f64 / (9 * k - 8) as f64;

        rem_cards[back_card_1] -= 1;
        card_counts_1[back_card_1] += 1;

        for back_card_2 in 0..9 {
            if rem_cards[back_card_2] == 0 {
                continue;
            }

            let prob2 = rem_cards[back_card_2] as f64 / (9 * k - 9) as f64;

            card_counts_2[back_card_2] += 1;

            if calc_score(&card_counts_1) > calc_score(&card_counts_2) {
                ans += prob1 * prob2;
            }

            card_counts_2[back_card_2] -= 1;
        }

        rem_cards[back_card_1] += 1;
        card_counts_1[back_card_1] -= 1;
    }

    println!("{}", ans);
}

fn calc_score(card_counts: &Vec<usize>) -> usize {
    (0..9)
        .map(|card| (card + 1) * 10_usize.pow(card_counts[card] as u32))
        .sum()
}
