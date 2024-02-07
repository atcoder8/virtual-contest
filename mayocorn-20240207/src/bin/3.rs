use proconio::input;

fn main() {
    input! {
        n: usize,
        mut hh: [usize; n],
    }

    let mut ans = 0;
    while hh.iter().any(|&h| h != 0) {
        let mut left = 0;
        while left < n {
            left = match (left..n).find(|&i| hh[i] != 0) {
                Some(left) => left,
                None => break,
            };
            let right = (left + 1..n).find(|&i| hh[i] == 0).unwrap_or(n);
            hh[left..right].iter_mut().for_each(|h| *h -= 1);
            left = right + 1;
            ans += 1;
        }
    }

    println!("{}", ans);
}
