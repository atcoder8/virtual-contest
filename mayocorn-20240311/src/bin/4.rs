use proconio::input;

fn main() {
    input! {
        n: usize,
        fff: [[usize; 10]; n],
        ppp: [[i64; 11]; n],
    }

    let calc_score = |bit: usize| {
        let mut open_num_each_store = vec![0; n];
        for i in 0..10 {
            if bit >> i & 1 == 1 {
                for store in 0..n {
                    open_num_each_store[store] += fff[store][i];
                }
            }
        }

        let mut score = 0;
        for store in 0..n {
            let open_num = open_num_each_store[store];
            score += ppp[store][open_num];
        }

        score
    };

    let ans = (1..1 << 10).map(calc_score).max().unwrap();
    println!("{}", ans);
}
