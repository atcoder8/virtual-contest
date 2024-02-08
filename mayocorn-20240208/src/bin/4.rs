use std::io::Write;

use proconio::input_interactive;

fn main() {
    input_interactive!(n: usize);

    let x = {
        let mut ok = n;
        let mut ng = 0_usize;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;

            println!("? 1 {mid} 1 {n}");
            input_interactive!(t: usize);

            if t < mid {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    };

    let y = {
        let mut ok = n;
        let mut ng = 0_usize;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;

            println!("? 1 {n} 1 {mid}");
            std::io::stdout().flush().unwrap();
            input_interactive!(t: usize);

            if t < mid {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    };

    println!("! {x} {y}");
    std::io::stdout().flush().unwrap();
}
