use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
    }

    let mut ans = 0;
    s.insert(0, 'J');
    ans = ans.max(joi_count(&s));
    s.remove(0);
    s.push('I');
    ans = ans.max(joi_count(&s));
    s.pop();

    let mut max_add = 0;
    let mut j_cnt = 0;
    let mut i_rem = s.iter().filter(|&&c| c == 'I').count();
    for &c in &s {
        match c {
            'J' => j_cnt += 1,
            'I' => i_rem -= 1,
            _ => {}
        }

        max_add = max_add.max(j_cnt * i_rem);
    }
    ans = ans.max(joi_count(&s) + max_add);

    println!("{}", ans);
}

fn char_to_usize(c: char) -> usize {
    match c {
        'J' => 0,
        'O' => 1,
        'I' => 2,
        _ => panic!(),
    }
}

fn joi_count(s: &[char]) -> usize {
    let mut dp = vec![0_usize; 3];
    for &c in s {
        let c_idx = char_to_usize(c);
        if c_idx == 0 {
            dp[0] += 1;
        } else {
            dp[c_idx] += dp[c_idx - 1];
        }
    }

    dp[2]
}
