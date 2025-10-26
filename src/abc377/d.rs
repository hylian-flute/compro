// https://atcoder.jp/contests/abc377/tasks/abc377_d

use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); n],
    };

    let mut r_to_l = HashMap::new();
    for (l, r) in lr.iter() {
        if let Some(backward_l) = r_to_l.get(r) {
            if *l >= *backward_l {
                r_to_l.insert(*r, *l);
            }
        } else {
            r_to_l.insert(*r, *l);
        }
    }

    let mut ans = 0usize;
    let mut added = 0usize;

    for i in 0..m {
        let end_at_i_count = (added + 1).min(if let Some(l) = r_to_l.get(&i) {
            i - *l
        } else {
            i + 1
        });

        ans += end_at_i_count;
        added = end_at_i_count;
    }

    println!("{}", ans);
}
