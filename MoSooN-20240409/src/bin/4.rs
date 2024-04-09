use itertools::enumerate;
use proconio::input;

const OFFSET: usize = 20000;

fn main() {
    input! {
        (n, x, y): (usize, i64, i64),
        aa: [usize; n],
    }

    let mut dpx = vec![false; OFFSET * 2 + 1];
    dpx[OFFSET + aa[0]] = true;

    let mut dpy = vec![false; OFFSET * 2 + 1];
    dpy[OFFSET] = true;

    for (i, &a) in enumerate(&aa).skip(1) {
        let mut next_dpx = vec![false; OFFSET * 2 + 1];
        let mut next_dpy = vec![false; OFFSET * 2 + 1];

        if i % 2 == 0 {
            for x in a..=2 * OFFSET - a {
                next_dpx[x] |= dpx[x - a] | dpx[x + a];
            }

            dpx = next_dpx;
        } else {
            for y in a..=2 * OFFSET - a {
                next_dpy[y] |= dpy[y - a] | dpy[y + a];
            }

            dpy = next_dpy;
        }
    }

    let ans = dpx[(x + OFFSET as i64) as usize] && dpy[(y + OFFSET as i64) as usize];
    println!("{}", if ans { "Yes" } else { "No" });
}
