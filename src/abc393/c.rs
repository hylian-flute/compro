// https://atcoder.jp/contests/abc393/tasks/abc393_c

use std::collections::HashSet;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut path_set = HashSet::<usize>::new();
    let mut result = 0usize;
    for (u, v) in uv {
        let (small, large) = if u < v { (u, v) } else { (v, u) };
        let idx = small * n + large;

        if small == large || path_set.contains(&idx) {
            result += 1;
        }

        path_set.insert(idx);
    }

    println!("{}", result);
}
