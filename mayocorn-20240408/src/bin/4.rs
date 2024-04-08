use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut set = BTreeSet::<(usize, usize)>::new();
    for qi in 0..q {
        input! {
            qt: usize,
        }

        match qt {
            1 => {
                input! {
                    x: usize,
                }

                set.insert((x, qi));
            }
            2 => {
                input! {
                    (x, k): (usize, usize),
                }

                let ans = set.range(..(x + 1, 0)).rev().nth(k - 1);
                match ans {
                    Some((val, _)) => println!("{}", val),
                    None => println!("-1"),
                }
            }
            3 => {
                input! {
                    (x, k): (usize, usize),
                }

                let ans = set.range((x, 0)..).nth(k - 1);
                match ans {
                    Some((val, _)) => println!("{}", val),
                    None => println!("-1"),
                }
            }
            _ => unreachable!(),
        }
    }
}
