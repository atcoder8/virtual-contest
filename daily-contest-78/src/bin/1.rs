use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        ccc: [[usize; 10]; 10],
        aaa: [[i64; w]; h],
    }

    let mut distances = ccc;
    for mid in 0..10 {
        for from in 0..10 {
            for to in 0..10 {
                let candidate_dist = distances[from][mid] + distances[mid][to];
                if candidate_dist < distances[from][to] {
                    distances[from][to] = candidate_dist;
                }
            }
        }
    }

    let ans: usize = aaa
        .iter()
        .map(|aa| {
            aa.iter()
                .map(|&a| if a != -1 { distances[a as usize][1] } else { 0 })
                .sum::<usize>()
        })
        .sum();
    println!("{}", ans);
}
