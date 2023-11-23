use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut pes = vec![];
    for _ in 0..n {
        input! {
            m: usize,
            pe: [(usize, usize); m],
        }

        pes.push(pe);
    }

    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    for pe in &pes {
        for &(p, e) in pe {
            let v = map.entry(p).or_insert((e, 0));
            if e > v.0 {
                *v = (e, 1);
            } else if e == v.0 {
                v.1 += 1;
            }
        }
    }

    let mut ans = 0_usize;
    let mut all = false;
    for pe in &pes {
        if pe.iter().any(|&(p, e)| map[&p] == (e, 1)) {
            ans += 1;
        } else {
            all = true;
        }
    }
    ans += all as usize;

    println!("{}", ans);
}
