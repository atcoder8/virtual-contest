use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let ans = (|| {
        for i in 0..n {
            let (x1, y1) = xy[i];

            for j in (i + 1)..n {
                let (x2, y2) = xy[j];

                for k in (j + 1)..n {
                    let (x3, y3) = xy[k];

                    if (y1 - y2) * (x1 - x3) == (y1 - y3) * (x1 - x2) {
                        return true;
                    }
                }
            }
        }

        false
    })();

    println!("{}", if ans { "Yes" } else { "No" });
}
