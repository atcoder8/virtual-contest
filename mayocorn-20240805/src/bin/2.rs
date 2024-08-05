use proconio::input;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; n],
        mut bb: [usize; m],
    }

    aa.sort_unstable();
    bb.sort_unstable();

    let mut sum_cost = 0;
    let mut iter_a = aa.iter();
    for &b in &bb {
        let Some(&cost) = iter_a.find(|&&a| a >= b) else {
            return None;
        };

        sum_cost += cost;
    }

    Some(sum_cost)
}
