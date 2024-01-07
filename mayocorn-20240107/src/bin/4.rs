use itertools::iproduct;
use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        (h, w): (usize, usize),
        ss: [Chars; h],
    }

    let mut hor_barriers = vec![vec![0]; h];
    let mut ver_barriers = vec![vec![0]; w];
    for (row, col) in iproduct!(0..h, 0..w) {
        if ss[row][col] == '#' {
            hor_barriers[row].push(col + 1);
            ver_barriers[col].push(row + 1);
        }
    }
    hor_barriers.iter_mut().for_each(|x| x.push(w + 1));
    ver_barriers.iter_mut().for_each(|x| x.push(h + 1));

    let mut ans = 0;
    for (row, col) in iproduct!(0..h, 0..w) {
        if ss[row][col] == '#' {
            continue;
        }

        let pos = hor_barriers[row].upper_bound(&(col + 1));
        let hor_num = hor_barriers[row][pos] - hor_barriers[row][pos - 1] - 1;

        let pos = ver_barriers[col].upper_bound(&(row + 1));
        let ver_num = ver_barriers[col][pos] - ver_barriers[col][pos - 1] - 1;

        ans = ans.max(hor_num + ver_num - 1);
    }

    println!("{}", ans);
}
