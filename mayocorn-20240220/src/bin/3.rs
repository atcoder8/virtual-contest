use proconio::input;

fn main() {
    input! {
        (n, m, x): (usize, usize, usize),
        caa: [[usize; m + 1]; n],
    }

    let is_ok = |bit: usize| {
        let mut skills = vec![0; m];
        for i in 0..n {
            if bit >> i & 1 == 1 {
                for j in 0..m {
                    skills[j] += caa[i][j + 1];
                }
            }
        }

        skills.iter().all(|&skill| skill >= x)
    };

    let ans = (0..1 << n)
        .filter(|&bit| is_ok(bit))
        .map(|bit| {
            (0..n)
                .filter(|&i| bit >> i & 1 == 1)
                .map(|i| caa[i][0])
                .sum::<usize>()
        })
        .min();
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
