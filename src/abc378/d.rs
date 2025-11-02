// https://atcoder.jp/contests/abc378/tasks/abc378_d

use proconio::input;
use proconio::marker::{Chars};
use std::collections::{HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h]
    };

    let point_to_index = |h_i: usize, w_i: usize| {
        h_i * w + w_i
    };

    let mut stack = Vec::<(usize, usize, usize, HashSet<usize>)>::new();
    for h_i in 0..h {
        for w_i in 0..w {
            stack.push((h_i, w_i, k, HashSet::new()));
        }
    }

    let mut ans = 0;
    while let Some((h_i, w_i, k_i, past)) = stack.pop() {
        if s[h_i][w_i] == '#' {
            continue;
        } else if past.get(&point_to_index(h_i, w_i)).is_some() {
            continue;
        } else if k_i == 0 {
            ans += 1;
            continue;
        }

        let mut new_past = past.clone();
        new_past.insert(point_to_index(h_i, w_i));
        if h_i >= 1 {
            stack.push((h_i - 1, w_i, k_i - 1, new_past.clone()));
        }
        if h_i < h - 1 {
            stack.push((h_i + 1, w_i, k_i - 1, new_past.clone()));
        }
        if w_i >= 1 {
            stack.push((h_i, w_i - 1, k_i - 1, new_past.clone()));
        }
        if w_i < w - 1 {
            stack.push((h_i, w_i + 1, k_i - 1, new_past.clone()));
        }
    }

    println!("{}", ans);
}

