use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut cnt = 0;
    let mut balls: Vec<(usize, usize)> = vec![];
    for &a in &aa {
        if balls.is_empty() || a != balls.last().unwrap().0 {
            balls.push((a, 1));
            cnt += 1;
        } else {
            let mut last = balls.pop().unwrap();
            last.1 += 1;
            cnt += 1;

            if last.1 == last.0 {
                cnt -= last.1;
            } else {
                balls.push(last);
            }
        }

        println!("{}", cnt);
    }
}
