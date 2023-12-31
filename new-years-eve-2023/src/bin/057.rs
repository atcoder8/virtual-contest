use proconio::input;

fn main() {
    input! {
        (ab, bc, _ca): (usize, usize, usize),
    }

    println!("{}", ab * bc / 2);
}
