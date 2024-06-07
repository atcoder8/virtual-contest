use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..10_usize.pow(6) {
        let s = i.to_string();
        let x = s
            .chars()
            .chain(s.chars())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        ans += (x <= n) as usize;
    }

    println!("{}", ans);
}
