use im_rc::{hashmap, HashMap};
use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut acc_block_num = vec![vec![0; n + 1]; n + 1];
    for (i, j) in iproduct!(0..n, 0..n) {
        if ss[i][j] == '#' {
            acc_block_num[i + 1][j + 1] += 1;
        }
    }
    for (i, j) in iproduct!(1..=n, 1..n) {
        acc_block_num[i][j + 1] += acc_block_num[i][j];
    }
    for (i, j) in iproduct!(1..n, 1..=n) {
        acc_block_num[i + 1][j] += acc_block_num[i][j];
    }

    let ans = rec(&ss, Rect::new(0, n, 0, n), &acc_block_num, &mut hashmap! {});
    println!("{}", ans);
}

fn count_block_num(acc_block_num: &[Vec<usize>], rect: Rect) -> usize {
    acc_block_num[rect.bottom][rect.right] + acc_block_num[rect.top][rect.left]
        - (acc_block_num[rect.top][rect.right] + acc_block_num[rect.bottom][rect.left])
}

fn rec(
    grid: &[Vec<char>],
    rect: Rect,
    acc_block_num: &[Vec<usize>],
    memo: &mut HashMap<Rect, usize>,
) -> usize {
    if rect.height() == 0 || rect.width() == 0 {
        return 0;
    }

    if let Some(&min_cost) = memo.get(&rect) {
        return min_cost;
    };

    let mut min_cost = rect.height().max(rect.width());

    let mut row = rect.top;
    while row < rect.bottom {
        let empty_row_num = (row..rect.bottom)
            .take_while(|&i| {
                count_block_num(acc_block_num, Rect::new(i, i + 1, rect.left, rect.right)) == 0
            })
            .count();

        if empty_row_num == 0 {
            row += 1;
            continue;
        }

        let sub_rect_1 = Rect::new(rect.top, row, rect.left, rect.right);
        let sub_rect_2 = Rect::new(row + empty_row_num, rect.bottom, rect.left, rect.right);

        min_cost = min_cost.min(
            rec(grid, sub_rect_1, acc_block_num, memo) + rec(grid, sub_rect_2, acc_block_num, memo),
        );

        row += empty_row_num;
    }

    let mut col = rect.left;
    while col < rect.right {
        let empty_col_num = (col..rect.right)
            .take_while(|&j| {
                count_block_num(acc_block_num, Rect::new(rect.top, rect.bottom, j, j + 1)) == 0
            })
            .count();

        if empty_col_num == 0 {
            col += 1;
            continue;
        }

        let sub_rect_1 = Rect::new(rect.top, rect.bottom, rect.left, col);
        let sub_rect_2 = Rect::new(rect.top, rect.bottom, col + empty_col_num, rect.right);

        min_cost = min_cost.min(
            rec(grid, sub_rect_1, acc_block_num, memo) + rec(grid, sub_rect_2, acc_block_num, memo),
        );

        col += empty_col_num;
    }

    memo.insert(rect, min_cost);

    min_cost
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rect {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl Rect {
    fn new(top: usize, bottom: usize, left: usize, right: usize) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    fn height(&self) -> usize {
        self.bottom - self.top
    }

    fn width(&self) -> usize {
        self.right - self.left
    }
}
