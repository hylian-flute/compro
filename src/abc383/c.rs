// https://atcoder.jp/contests/abc383/tasks/abc383_c

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: i32,
        s: [Chars; h]
    };

    // 加湿度合い
    let mut h_levels = vec![vec![0i32; w]; h];

    let mut deque = VecDeque::<(usize, usize, i32)>::new();

    for h_i in 0..h {
        for w_i in 0..w {
            if s[h_i][w_i] == 'H' {
                deque.push_back((h_i, w_i, d + 1));
            }
        }
    }

    while let Some((h_i, w_i, d_hw)) = deque.pop_front() {
        h_levels[h_i][w_i] = d_hw;

        if d_hw <= 1 {
            continue;
        }

        for (h_diff, w_diff) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_h = h_i as i32 + h_diff;
            let next_w = w_i as i32 + w_diff;

            if next_h >= 0
                && next_h < h as i32
                && next_w >= 0
                && next_w < w as i32
                && s[next_h as usize][next_w as usize] == '.'
                && h_levels[next_h as usize][next_w as usize] < d_hw - 1
            {
                deque.push_back((next_h as usize, next_w as usize, d_hw - 1));
            }
        }
    }

    let mut result = 0i32;
    for h_i in 0..h {
        for w_i in 0..w {
            if h_levels[h_i][w_i] > 0 {
                result += 1;
            }
        }
    }

    println!("{:}", result);
}
