use im_rc::HashSet;
use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
    }

    let mut best: Option<Best> = None;
    let mut used = HashSet::<String>::new();
    for (i, (s, t)) in enumerate(st) {
        if used.contains(&s) {
            continue;
        }

        used.insert(s);

        if best.is_none() || t > best.unwrap().score {
            best = Some(Best { idx: i, score: t });
        }
    }

    println!("{}", best.unwrap().idx + 1);
}

#[derive(Debug, Clone, Copy)]
struct Best {
    idx: usize,
    score: usize,
}
