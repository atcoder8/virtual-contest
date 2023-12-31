use proconio::input;

fn main() {
    input! {
        n: usize,
        scores_per_examinee: [[f64; 5]; n],
    }

    let calc_total_score =
        |scores: &[f64]| scores[..4].iter().sum::<f64>() + scores[4] * 110.0 / 900.0;

    let ans = scores_per_examinee
        .iter()
        .map(|scores| calc_total_score(scores))
        .max_by(|x, y| x.partial_cmp(&y).unwrap())
        .unwrap();
    println!("{}", ans);
}
