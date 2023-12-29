use proconio::input;

fn main() {
    input! {
        n: usize,
        hh: [usize; n],
    }

    let mut cur = 0;
    for &h in &hh {
        if h <= cur {
            break;
        }

        cur = h;
    }

    println!("{}", cur);
}
