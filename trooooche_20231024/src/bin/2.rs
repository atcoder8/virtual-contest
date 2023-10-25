use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (_n, k): (usize, usize),
        s: Chars,
    }

    println!("{}", solve(k, s));
}

fn solve(k: usize, s: Vec<char>) -> char {
    let mut winners = s.iter().map(|&hand| char_to_u8(hand)).collect_vec();
    for _ in 0..k {
        if winners.len() % 2 == 1 {
            winners.extend(winners.clone());
        }
        winners = winners
            .chunks_mut(2)
            .map(|hands| select_win_hand(hands))
            .collect_vec();
    }

    u8_to_char(winners[0])
}

fn char_to_u8(hand: char) -> u8 {
    match hand {
        'R' => 0,
        'P' => 1,
        'S' => 2,
        _ => panic!(),
    }
}

fn u8_to_char(hand: u8) -> char {
    match hand {
        0 => 'R',
        1 => 'P',
        2 => 'S',
        _ => panic!(),
    }
}

fn select_win_hand(hands: &mut [u8]) -> u8 {
    if hands[0] > hands[1] {
        hands.swap(0, 1);
    }

    hands[(hands[1] - hands[0] <= 1) as usize]
}
