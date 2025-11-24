// https://atcoder.jp/contests/abc380/tasks/abc380_e

use std::collections::BTreeMap;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut color_from_separation_idx = (0..n)
        .map(|idx| (idx, idx))
        .collect::<BTreeMap<usize, usize>>();

    let mut count_from_color = vec![1usize; n];

    for _ in 0..q {
        input! {
            command: usize,
        };

        match command {
            1 => {
                input! {
                    x: Usize1,
                    c: Usize1,
                };

                let (left_idx, left_color) = {
                    let (idx, color) = color_from_separation_idx.range(0..=x).last().unwrap();
                    (*idx, *color)
                };
                if left_idx == 0 {
                    color_from_separation_idx.insert(left_idx, c);
                } else {
                    let further_left_idx_and_color =
                        color_from_separation_idx.range(0..left_idx).last().unwrap();
                    if *further_left_idx_and_color.1 == c {
                        color_from_separation_idx.remove(&left_idx);
                    } else {
                        color_from_separation_idx.insert(left_idx, c);
                    }
                }

                let right_idx_and_color = color_from_separation_idx
                    .range((x + 1)..n)
                    .next()
                    .map(|(idx, color)| (*idx, *color));
                if let Some(v) = right_idx_and_color {
                    if v.1 == c {
                        color_from_separation_idx.remove(&v.0);
                    };
                };

                let count = match right_idx_and_color {
                    Some(v) => v.0,
                    None => n,
                } - left_idx;
                count_from_color[c] += count;
                count_from_color[left_color] -= count;
            }
            _ => {
                input! {
                    c: Usize1,
                };
                println!("{}", count_from_color[c]);
            }
        };
    }
}
