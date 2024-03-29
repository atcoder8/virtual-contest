use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        s: usize,
    }

    let mut pool = HashSet::new();
    let mut ans = 0;
    let mut t = s;
    while !pool.contains(&t) {
        ans += 1;
        pool.insert(t);
        t = if t % 2 == 0 { t / 2 } else { 3 * t + 1 };
    }
    ans += 1;

    println!("{}", ans);
}
