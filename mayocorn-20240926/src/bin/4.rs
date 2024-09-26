use proconio::input;

fn main() {
    input! {
        (n, l, r): (usize, i64, i64),
        aa: [i64; n],
    }

    let mut acc_left = vec![0; n + 1];
    let mut acc_right = vec![0; n + 1];
    for i in 0..n {
        acc_left[i + 1] = acc_left[i] + aa[i];
        acc_right[n - 1 - i] = acc_right[n - i] + aa[n - 1 - i];
    }

    let mut left_decrease = vec![0_i64; n + 1];
    let mut right_decrease = vec![0_i64; n + 1];
    for replace_len in 1..=n {
        left_decrease[replace_len] =
            left_decrease[replace_len - 1].min(l * replace_len as i64 - acc_left[replace_len]);
        right_decrease[n - replace_len] = right_decrease[n - replace_len + 1]
            .min(r * replace_len as i64 - acc_right[n - replace_len]);
    }

    let sum_a = aa.iter().sum::<i64>();
    let ans = (0..=n)
        .map(|replace_len| sum_a + left_decrease[replace_len] + right_decrease[replace_len])
        .min()
        .unwrap();
    println!("{}", ans);
}
