use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        d: usize,
    }

    let calc_cost = |x: usize| {
        let square_x = x.pow(2);

        if square_x >= d {
            return square_x - d;
        }

        let sub = d - square_x;

        let mut ge = sub.sqrt() + 1;
        let mut lt = 0_usize;

        while ge.abs_diff(lt) > 1 {
            let mid = (ge + lt) / 2;

            if mid.pow(2) >= sub {
                ge = mid;
            } else {
                lt = mid;
            }
        }

        (ge.pow(2) - sub).min(sub - lt.pow(2))
    };

    let ans = (0_usize..=d.sqrt() + 1).map(calc_cost).min().unwrap();
    println!("{}", ans);
}
