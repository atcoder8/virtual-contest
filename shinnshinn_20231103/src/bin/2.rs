use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: Bytes,
    }

    let mut lt = None;
    let mut ge = 0;
    for &d in n.iter().rev() {
        let d = (d - b'0') as usize;

        let mut next_lt = None;
        let mut next_ge = None;

        if let Some(cost) = lt {
            // lt -> lt
            {
                let cand_cost = cost + 9 - d;
                if next_lt.is_none() || cand_cost < next_lt.unwrap() {
                    next_lt = Some(cand_cost);
                }
            }

            // lt -> ge
            if d < 9 {
                let cand_cost = cost + d + 1;
                if next_ge.is_none() || cand_cost < next_ge.unwrap() {
                    next_ge = Some(cand_cost);
                }
            }
        }

        {
            // ge -> lt
            if d > 0 {
                let cand_cost = ge + 10 - d;
                if next_lt.is_none() || cand_cost < next_lt.unwrap() {
                    next_lt = Some(cand_cost);
                }
            }

            // ge -> ge
            {
                let cand_cost = ge + d;
                if next_ge.is_none() || cand_cost < next_ge.unwrap() {
                    next_ge = Some(cand_cost);
                }
            }
        }

        lt = next_lt;
        ge = next_ge.unwrap();
    }

    let ans = (lt.unwrap() + 1).min(ge);
    println!("{}", ans);
}
