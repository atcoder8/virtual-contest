use proconio::input;

fn main() {
    input! {
        n: usize,
        benefit_mat: [[usize; n]; 1 << n],
    }

    // 最初、各参加者が互いに異なるグループに所属していて、勝負ごとに隣り合うグループが併合されるとする。
    // t回目の勝負の後、2^t人を含むグループが2^(N-t)個存在する。
    // dp[i]をfloor(i/(2^t))番目のグループにおいて人iが勝ち抜いたときのグループ内の人iを除く参加者が獲得する賞金の合計の最大値とする。(番号は0-based)
    // 1回目の勝負が終わった後から考える。このとき、既に敗退した人の賞金額は0である。
    // 勝負を行い、2つのグループが併合されるとき、ここまで勝ち抜いた人のうち1人が敗退し、賞金額が決定する。
    // dp[i]を更新するとき、人iを含まない側のグループに所属する人が敗北するので、その賞金額を加算した値の最大値を加算する。
    let mut dp = vec![0; 1 << n];
    for t in 1..n {
        let group_size = 1 << t;

        for left_group_pos in (0..1 << n).step_by(2 * group_size) {
            let right_group_pos = left_group_pos + group_size;

            let max_lose_left_group_score = (left_group_pos..left_group_pos + group_size)
                .map(|i| dp[i] + benefit_mat[i][t - 1])
                .max()
                .unwrap();

            let max_lose_right_group_score = (right_group_pos..right_group_pos + group_size)
                .map(|i| dp[i] + benefit_mat[i][t - 1])
                .max()
                .unwrap();

            for i in left_group_pos..left_group_pos + group_size {
                dp[i] += max_lose_right_group_score;
            }

            for i in right_group_pos..right_group_pos + group_size {
                dp[i] += max_lose_left_group_score;
            }
        }
    }

    let ans = (0..1 << n)
        .map(|i| dp[i] + benefit_mat[i][n - 1])
        .max()
        .unwrap();
    println!("{}", ans);
}
