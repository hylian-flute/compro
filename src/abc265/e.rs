// https://atcoder.jp/contests/abc265/tasks/abc265_e

use proconio::input;
use std::collections::{HashMap, HashSet};

const MOD_C: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        a_to_f: [(i32, i32); 3],
        xy: [(i32, i32); m],
    };

    let xy_set = xy.into_iter().collect::<HashSet<_>>();

    let mut available_points = vec![HashSet::<(i32, i32)>::new(); n + 1];
    let mut search_stack = vec![(0usize, (0i32, 0i32))];
    while let Some((warp_count, (x, y))) = search_stack.pop() {
        if xy_set.contains(&(x, y)) {
            continue;
        }
        if available_points[warp_count].contains(&(x, y)) {
            continue;
        }
        available_points[warp_count].insert((x, y));
        if warp_count >= n {
            continue;
        }

        for (x_diff, y_diff) in a_to_f.iter() {
            search_stack.push((warp_count + 1, (x + x_diff, y + y_diff)));
        }
    }

    let mut pattern_count_from_point = HashMap::<(usize, (i32, i32)), usize>::new();
    for warp_count in (0..=n).rev() {
        for (x, y) in available_points[warp_count].iter() {
            if warp_count == n {
                pattern_count_from_point.insert((warp_count, (*x, *y)), 1);
            } else {
                let mut pattern_count = 0usize;
                for (x_diff, y_diff) in a_to_f.iter() {
                    if let Some(pattern) =
                        pattern_count_from_point.get(&(warp_count + 1, (x + x_diff, y + y_diff)))
                    {
                        pattern_count = (pattern_count + pattern) % MOD_C;
                    }
                }
                pattern_count_from_point.insert((warp_count, (*x, *y)), pattern_count);
            }
        }
    }

    println!("{}", pattern_count_from_point.get(&(0, (0, 0))).unwrap());
}
