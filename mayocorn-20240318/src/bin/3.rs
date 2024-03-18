use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let mut stack: Vec<char> = vec![];
    let mut left_cnt = 0;
    for c in s.chars() {
        if c == '(' {
            left_cnt += 1;
        } else if c == ')' && left_cnt != 0 {
            while let Some(pop_c) = stack.pop() {
                if pop_c == '(' {
                    break;
                }
            }

            left_cnt -= 1;

            continue;
        }

        stack.push(c);
    }

    let ans = stack.iter().join("");
    println!("{}", ans);
}
