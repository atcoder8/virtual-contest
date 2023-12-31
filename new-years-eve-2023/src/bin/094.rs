use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let sum = n * (n + 1) / 2;
    let ans = primality_test(sum);
    println!("{}", if ans { "WANWAN" } else { "BOWWOW" });
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
