use proconio::input;

fn main() {
    input! {
        p: usize,
        n: usize,
    }

    let mut rems = vec![0_usize; p];
    for i in 0..p {
        rems[pow_mod(i, n, p)] += 1;
    }

    let mut ans = 0;
    for x in 0..p {
        if rems[x] == 0 {
            continue;
        }

        for y in 0..p {
            let z = (x + y) % p;
            ans += rems[x] * rems[y] * rems[z];
        }
    }

    println!("{}", ans);
}

fn pow_mod(x: usize, n: usize, m: usize) -> usize {
    let mut x = x % m;
    x %= m;
    let mut ret = 1 % m;
    let mut n = n;

    while n != 0 {
        if n & 1 == 1 {
            ret = ret * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }

    ret
}
