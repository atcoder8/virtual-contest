use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let digit_num = n.len();

    let is_ok = |bit: usize| {
        (0..digit_num)
            .filter(|&i| bit >> i & 1 == 1)
            .map(|i| n[i].to_digit(10).unwrap())
            .sum::<u32>()
            % 3
            == 0
    };

    let ans = (1..1 << digit_num)
        .filter(|&bit| is_ok(bit))
        .map(|bit| digit_num - bit.count_ones() as usize)
        .min();
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
