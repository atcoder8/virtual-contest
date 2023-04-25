use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        ab: [(Usize1, Usize1); n],
    }

    let mut imos = vec![vec![0_i64; 5001]; 5001];
    for &(a, b) in &ab {
        imos[a][b] += 1;
        imos[a][(b + k + 1).min(5000)] -= 1;
        imos[(a + k + 1).min(5000)][b] -= 1;
        imos[(a + k + 1).min(5000)][(b + k + 1).min(5000)] += 1;
    }

    for i in 0..5000 {
        for j in 0..5000 {
            imos[i][j + 1] += imos[i][j];
        }
    }

    for i in 0..5000 {
        for j in 0..=5000 {
            imos[i + 1][j] += imos[i][j];
        }
    }

    let mut ans = 0;
    for i in 0..5000 {
        for j in 0..5000 {
            ans = ans.max(imos[i][j]);
        }
    }

    println!("{}", ans);
}
