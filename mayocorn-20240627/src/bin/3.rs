use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                (x, c): (usize, usize),
            }

            queue.push_back((x, c));
        } else {
            input! {
                c: usize,
            }

            let mut sum = 0;
            let mut rem = c;

            while let Some((val, cnt)) = queue.front_mut() {
                let consume = rem.min(*cnt);
                rem -= consume;
                *cnt -= consume;

                sum += *val * consume;

                if *cnt == 0 {
                    queue.pop_front();
                }

                if rem == 0 {
                    break;
                }
            }

            println!("{}", sum);
        }
    }
}
