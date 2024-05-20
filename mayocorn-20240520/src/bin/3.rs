use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = rec(n, &mut HashMap::new());
    println!("{}", ans);
}

fn rec(x: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if x <= 1 {
        return 0;
    }

    if let Some(&cost) = memo.get(&x) {
        return cost;
    }

    let half = x / 2;
    let cost = x + rec(half, memo) + rec(x - half, memo);
    memo.insert(x, cost);

    cost
}
