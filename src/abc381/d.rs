// https://atcoder.jp/contests/abc381/tasks/abc381_d

use proconio::input;
use std::{cmp::max, collections::HashMap};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let (mut left, mut right) = (0usize, 0usize);
    let mut appeared_index_from_number = HashMap::<usize, usize>::new();
    let mut max_length = 0usize;

    while right + 1 < n {
        if a[right] == a[right + 1] {
            let appeared_index = appeared_index_from_number.get(&a[right]).map(|v| *v);
            if let Some(i) = appeared_index {
                left = max(i + 2, left);
            }
            appeared_index_from_number.insert(a[right], right);
            right += 2;
        } else {
            appeared_index_from_number.clear();
            left = max(right.saturating_sub(1), left + 1);
            right = left;
        }

        max_length = max(right - left, max_length);
    }

    println!("{}", max_length);
}
