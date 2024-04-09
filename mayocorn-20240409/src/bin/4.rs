use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let count = |k: usize| {
        let digits = k
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec();
        let head = digits[0];
        let tail = *digits.last().unwrap();

        if tail == 0 {
            return 0;
        }

        let mut cnt = (head == tail && head <= n) as usize;
        for digit_num in 2.. {
            let min = tail * 10_usize.pow(digit_num - 1) + head;

            if min > n {
                break;
            }

            let max = (tail + 1) * 10_usize.pow(digit_num - 1) - 10 + head;

            cnt += (max.min(n) - min) / 10 + 1;
        }

        cnt
    };

    let ans = (1..=n).map(count).sum::<usize>();
    println!("{}", ans);
}
