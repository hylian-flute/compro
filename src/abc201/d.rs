use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h]
    };

    let mut diffs = vec![vec![0_i32; w]; h];
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            let is_takahashi_turn = (y + x) % 2 == 1;
            let this_diff = if y == 0 && x == 0 {
                0_i32
            } else {
                if is_takahashi_turn == (a[y][x] == '+') {
                    1_i32
                } else {
                    -1_i32
                }
            };
            let mut next_diff = Option::<i32>::None;
            if y + 1 < h {
                let bottom_diff = diffs[y + 1][x];
                if let Some(d) = next_diff {
                    next_diff = Some(if is_takahashi_turn {
                        d.min(bottom_diff)
                    } else {
                        d.max(bottom_diff)
                    });
                } else {
                    next_diff = Some(bottom_diff);
                }
            };
            if x + 1 < w {
                let right_diff = diffs[y][x + 1];
                if let Some(d) = next_diff {
                    next_diff = Some(if is_takahashi_turn {
                        d.min(right_diff)
                    } else {
                        d.max(right_diff)
                    });
                } else {
                    next_diff = Some(right_diff);
                }
            };
            diffs[y][x] = this_diff + next_diff.unwrap_or(0);
        }
    }

    let result = if diffs[0][0] > 0 {
        "Takahashi"
    } else if diffs[0][0] == 0 {
        "Draw"
    } else {
        "Aoki"
    };
    println!("{}", result);
}
