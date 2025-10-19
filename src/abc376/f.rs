// https://atcoder.jp/contests/abc376/tasks/abc376_f

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(String, Usize1); q],
    };

    let count = |q_i: usize, position: usize, count_memo: &Vec<Vec<usize>>| {
        let (target_position, another_position) = if q_i == 0 {
            if ht[q_i].0 == "L" {
                (0, 1)
            } else {
                (1, 0)
            }
        } else if ht[q_i].0 == ht[q_i - 1].0 {
            (ht[q_i - 1].1, position)
        } else {
            (position, ht[q_i - 1].1)
        };

        if target_position == another_position {
            return 0;
        };

        let (min_up, another_position_up) = {
            let t = ht[q_i].1;
            let tmp_t = if t < target_position { t + n } else { t };
            let tmp_another_position = if another_position < target_position {
                another_position + n
            } else {
                another_position
            };

            if tmp_t < tmp_another_position {
                (tmp_t - target_position, another_position)
            } else {
                (
                    (tmp_another_position - target_position - 1)
                        + 2 * (tmp_t - tmp_another_position + 1),
                    (t + 1) % n,
                )
            }
        };

        let (min_down, another_position_down) = {
            let t = ht[q_i].1;
            let tmp_target_position = if target_position < t {
                target_position + n
            } else {
                target_position
            };
            let tmp_another_position = if another_position < t {
                another_position + n
            } else {
                another_position
            };

            if tmp_target_position < tmp_another_position {
                (tmp_target_position - t, another_position)
            } else {
                (
                    (tmp_target_position - tmp_another_position - 1)
                        + 2 * (tmp_another_position - t + 1),
                    (t + n - 1) % n,
                )
            }
        };

        if q_i == q - 1 {
            min_up.min(min_down)
        } else {
            (min_up + count_memo[q_i + 1][another_position_up])
                .min(min_down + count_memo[q_i + 1][another_position_down])
        }
    };

    let mut count_memo = vec![vec![0; n]; q];
    for q_i in (0..q).rev() {
        for position in 0..n {
            count_memo[q_i][position] = count(q_i, position, &count_memo);
        }
    }

    println!("{}", count_memo[0][0]);
}
