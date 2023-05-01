use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let mut init_forward_cnt = 0;
    let mut hor_cnt = vec![0; h];
    let mut ver_cnt = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            if sss[i][j] == '#' {
                init_forward_cnt += 1;
                hor_cnt[i] += 1;
                ver_cnt[j] += 1;
            }
        }
    }

    let forward_num = init_forward_cnt
        + (0..h)
            .map(|i| {
                w as i64 - 2 * hor_cnt[i]
                    + (0..w)
                        .map(|j| h as i64 - 2 * ver_cnt[j] + 4 * (sss[i][j] == '#') as i64 - 2)
                        .min()
                        .unwrap()
            })
            .max()
            .unwrap();
    println!("{} {}", forward_num, (h * w) as i64 - forward_num);
}
