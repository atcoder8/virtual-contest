use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    let groups = [vec![1, 3, 5, 7, 8, 10, 12], vec![4, 6, 9, 11], vec![2]];

    let group1 = groups.iter().position(|group| group.contains(&x)).unwrap();
    let group2 = groups.iter().position(|group| group.contains(&y)).unwrap();

    println!("{}", if group1 == group2 { "Yes" } else { "No" });
}
