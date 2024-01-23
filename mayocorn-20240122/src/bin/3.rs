use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut ll: [usize; n],
    }

    ll.sort_unstable();

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans += ll.lower_bound(&(ll[i] + ll[j])) - 1 - j;
        }
    }

    println!("{}", ans);
}
