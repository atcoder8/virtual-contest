use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = primality_test(n);
    println!("{}", if ans { "YES" } else { "NO" });
}

fn primality_test(n: usize) -> bool {
    if n <= 1 {
        return false;
    }

    for i in (2..).take_while(|x| x * x <= n) {
        if n % i == 0 {
            return false;
        }
    }

    true
}
