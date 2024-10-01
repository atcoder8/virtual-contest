use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut p = 0;
    let mut bits = 0_u8;
    for &a in &aa {
        bits |= 1;
        bits <<= a;
        p += (bits >> 4).count_ones();
        bits &= 0b1111;
    }
    println!("{}", p);
}
