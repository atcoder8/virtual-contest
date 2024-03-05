use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut cnt = 0;
    let mut used = HashSet::<usize>::new();
    for a in (2_usize..).take_while(|&a| a.pow(2) <= n) {
        for b in (2_u32..).take_while(|&b| a.pow(b) <= n) {
            let pow = a.pow(b);

            if used.contains(&pow) {
                continue;
            }

            used.insert(pow);

            cnt += 1;
        }
    }

    let ans = n - cnt;
    println!("{}", ans);
}
