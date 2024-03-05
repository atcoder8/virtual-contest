use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    let ans = rec(x, y, &mut HashMap::new());
    println!("{}", ans);
}

fn rec(x: usize, cur_y: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(&cnt) = memo.get(&cur_y) {
        return cnt;
    }

    if cur_y <= x {
        return x - cur_y;
    }

    let cnt = if cur_y % 2 == 0 {
        rec(x, cur_y / 2, memo) + 1
    } else {
        rec(x, (cur_y + 1) / 2, memo).min(rec(x, (cur_y - 1) / 2, memo)) + 2
    }
    .min(cur_y - x);

    memo.insert(cur_y, cnt);

    cnt
}
