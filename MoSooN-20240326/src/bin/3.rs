use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut pool = HashSet::<usize>::new();
    for a in (2_usize..).take_while(|a| a.pow(2) <= n) {
        let mut t = a.pow(2);
        while t <= n {
            pool.insert(t);
            t *= a;
        }
    }

    println!("{}", n - pool.len());
}
