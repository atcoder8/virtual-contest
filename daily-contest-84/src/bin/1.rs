use proconio::input;

fn main() {
    input! {
        n: usize,
        tt: [usize; n],
    }

    let mut a = 0;
    for &t in &tt {
        let bit = 1_usize << t;
        a &= !(bit - 1);
        a += bit;
        if a & bit == 0 {
            a += bit;
        }
    }

    println!("{}", a);
}
