use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        s: Chars,
    }

    let x_num_per_cycle = s.iter().filter(|&&c| c == 'x').count();

    let is_ok = |key: usize| {
        let (q, r) = (k / x_num_per_cycle, k % x_num_per_cycle);

        if q == m {
            return n * m >= key;
        }

        let mut max = 0;
        let mut rem = r;
        let mut right = 0;
        for left in 0..n {
            if right < left {
                right = left;
            }

            while q + (right + n - 1) / n <= m {
                if s[right % n] == 'x' {
                    if rem == 0 {
                        break;
                    }

                    rem -= 1;
                }

                right += 1;
            }

            max = max.max(right - left);

            if left < right && s[left] == 'x' {
                rem += 1;
            }
        }

        n * q + max >= key
    };

    let mut ok = 0;
    let mut ng = n * m + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
