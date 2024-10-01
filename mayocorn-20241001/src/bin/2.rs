use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut ans = 1_usize;
    for b in (2_usize..).take_while(|b| b.pow(2) <= x) {
        let mut raised = b.pow(2);
        while raised <= x {
            ans = ans.max(raised);
            raised *= b;
        }
    }
    println!("{}", ans);
}
