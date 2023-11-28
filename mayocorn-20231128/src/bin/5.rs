use itertools::enumerate;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, q, x): (usize, usize, usize),
        ww: [usize; n],
        kk: [usize; q],
    }

    let sum_w: usize = ww.iter().sum();
    let max_k = *kk.iter().max().unwrap();
    let log = max_k.ilog2() as usize;

    let mut acc = vec![0; 2 * n + 1];
    for (i, &w) in enumerate(ww.iter().chain(&ww)) {
        acc[i + 1] += acc[i] + w;
    }

    // req[i]: i番目のじゃがいもから始めて1箱詰めるときの必要個数
    let mut req = vec![0; n];
    for start in 0..n {
        let (q, r) = (x / sum_w, x % sum_w);
        let stop = acc.lower_bound(&(acc[start] + r));
        req[start] = q * n + stop - start;
    }

    // doubling[i][j]: j番目(0-based)のじゃがいもから始めて2^i箱詰めた直後の開始地点
    let mut doubling = vec![vec![0; n]; log + 1];
    for start in 0..n {
        doubling[0][start] = (start + req[start]) % n;
    }
    for i in 0..log {
        for start in 0..n {
            doubling[i + 1][start] = doubling[i][doubling[i][start]];
        }
    }

    for k in kk {
        let mut cur = 0;
        for exp in (0..).take_while(|exp| (k - 1) >> exp != 0) {
            if (k - 1) >> exp & 1 == 1 {
                cur = doubling[exp][cur];
            }
        }

        // cur: k-1箱詰めた直後の開始地点
        println!("{}", req[cur]);
    }
}
