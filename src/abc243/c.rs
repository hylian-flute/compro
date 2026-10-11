use std::collections::HashMap;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
        s: Chars,
    };

    let mut memo = HashMap::<(i32, char), i32>::new();
    for i in 0..n {
        let (x, y) = xy[i];
        let s_i = s[i];
        if let Some(cur_x) = memo.get_mut(&(y, s_i)) {
            if (s_i == 'R' && x < *cur_x) || (s_i == 'L' && x > *cur_x) {
                *cur_x = x;
            }
        } else {
            memo.insert((y, s_i), x);
        }
    }

    for i in 0..n {
        let (x, y) = xy[i];
        let s_i = s[i];
        if let Some(target_x) = memo.get(&(y, if s_i == 'R' { 'L' } else { 'R' })) {
            if (s_i == 'R' && *target_x > x) || (s_i == 'L' && *target_x < x) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
