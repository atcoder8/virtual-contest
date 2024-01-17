use proconio::input;

fn main() {
    input! {
        n: usize,
        tlr: [(usize, usize, usize); n],
    }

    let mut ans = 0;
    let mut sections = vec![];
    for &(t, mut l, mut r) in &tlr {
        (l, r) = match t {
            1 => (2 * l, 2 * r),
            2 => (2 * l, 2 * r - 1),
            3 => (2 * l + 1, 2 * r),
            4 => (2 * l + 1, 2 * r - 1),
            _ => unreachable!(),
        };

        ans += sections
            .iter()
            .filter(|&&(other_l, other_r)| l <= other_r && r >= other_l)
            .count();

        sections.push((l, r));
    }

    println!("{}", ans);
}
